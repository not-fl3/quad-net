use std::net::ToSocketAddrs;
use std::net::{TcpListener, TcpStream};
use std::time::{Duration, Instant};

use std::sync::{Arc, Mutex};

pub struct Settings<F, F1, S>
where
    F: Fn(Sender, &mut S, Vec<u8>) + Send + 'static,
    F1: Fn(Sender, &S) + Send + 'static,
    S: Default + Send,
{
    pub on_message: F,
    pub on_timer: F1,
    pub timer: Option<Duration>,

    pub _marker: std::marker::PhantomData<S>,
}

pub enum Sender<'a> {
    WebSocket(&'a ws::Sender),
    Tcp(&'a mut TcpStream),
}

impl<'a> Sender<'a> {
    pub fn send(&mut self, data: &[u8]) -> Option<()> {
        use std::io::Write;

        match self {
            Sender::WebSocket(out) => {
                out.send(data).ok()?;
            }
            Sender::Tcp(stream) => {
                stream.write(&[data.len() as u8]).ok()?;
                stream.write(data).ok()?;
            }
        }

        Some(())
    }
}

#[cfg(feature = "nanoserde")]
impl<'a> Sender<'a> {
    pub fn send_bin<T: nanoserde::SerBin>(&mut self, data: &T) -> Option<()> {
        self.send(&nanoserde::SerBin::serialize_bin(data))
    }
}

pub fn listen<A, A1, F, F1, S>(tcp_addr: A, ws_addr: A1, settings: Settings<F, F1, S>)
where
    A: ToSocketAddrs + std::fmt::Debug + Send,
    A1: ToSocketAddrs + std::fmt::Debug + Send + 'static,
    F: Fn(Sender, &mut S, Vec<u8>) + Send + 'static,
    F1: Fn(Sender, &S) + Send + 'static,
    S: Default + Send + 'static,
{
    let on_message = Arc::new(Mutex::new(settings.on_message));
    let on_timer = Arc::new(Mutex::new(settings.on_timer));
    let timer = settings.timer;

    struct WsHandler<
        S: Default,
        F: Fn(Sender, &mut S, Vec<u8>) + Send + 'static,
        F1: Fn(Sender, &S) + Send + 'static,
    > {
        out: ws::Sender,
        state: S,
        on_message: Arc<Mutex<F>>,
        on_timer: Arc<Mutex<F1>>,
        timeout: Option<Duration>,
    }

    impl<
            S: Default,
            F: Fn(Sender, &mut S, Vec<u8>) + Send + 'static,
            F1: Fn(Sender, &S) + Send + 'static,
        > ws::Handler for WsHandler<S, F, F1>
    {
        fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
            let data = msg.into_data();
            (self.on_message.lock().unwrap())(Sender::WebSocket(&self.out), &mut self.state, data);
            Ok(())
        }

        fn on_open(&mut self, _: ws::Handshake) -> ws::Result<()> {
            if let Some(timeout) = self.timeout {
                self.out.timeout(timeout.as_millis() as _, ws::util::Token(1))?;
            }
            Ok(())
        }
        fn on_timeout(&mut self, _: ws::util::Token) -> ws::Result<()> {
            if let Some(timeout) = self.timeout {
                self.out.timeout(timeout.as_millis() as _, ws::util::Token(1))?;
                (self.on_timer.lock().unwrap())(Sender::WebSocket(&self.out), &self.state);
            }
            Ok(())
        }
    }

    std::thread::spawn({
        let on_message = on_message.clone();
        let on_timer = on_timer.clone();

        move || {
            ws::Builder::new()
                .with_settings(ws::Settings {
                    timer_tick_millis: 10,
                    tcp_nodelay: true,
                    ..ws::Settings::default()
                })
                .build(move |out| {
                    let on_message = on_message.clone();
                    let on_timer = on_timer.clone();

                    WsHandler {
                        out,
                        state: S::default(),
                        on_message,
                        on_timer,
                        timeout: timer,
                    }
                })
                .unwrap()
                .listen(ws_addr)
                .unwrap();
        }
    });

    let listener = TcpListener::bind(tcp_addr).unwrap();
    for stream in listener.incoming() {
        let on_message = on_message.clone();
        let on_timer = on_timer.clone();

        std::thread::spawn(move || {
            let mut stream = stream.unwrap();
            stream.set_nodelay(true).unwrap();
            stream.set_nonblocking(true).unwrap();
            let mut message_reader = crate::protocol::MessageReader::new();
            let mut state = S::default();

            let mut time = Instant::now();
            loop {
                if let Some(message) = message_reader.next(&mut stream) {
                    (on_message.lock().unwrap())(Sender::Tcp(&mut stream), &mut state, message);
                }

                if let Some(timer) = timer {
                    if time.elapsed() >= timer {
                        time = Instant::now();

                        (on_timer.lock().unwrap())(Sender::Tcp(&mut stream), &state);
                    }
                }
            }
        });
    }
}
