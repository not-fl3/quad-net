use std::net::ToSocketAddrs;

use std::net::TcpStream;
use std::sync::mpsc::{self, Receiver};

use super::error::Error;
use crate::protocol::MessageReader;

pub struct TcpSocket {
    stream: TcpStream,
    rx: Receiver<Vec<u8>>,
}

impl TcpSocket {
    pub fn send(&mut self, data: &[u8]) {
        use std::io::Write;

        self.stream.write(&[data.len() as u8]).unwrap();
        self.stream.write(data).unwrap();
    }

    pub fn try_recv(&mut self) -> Option<Vec<u8>> {
        self.rx.try_recv().ok()
    }
}

impl TcpSocket {
    pub fn connect<A: ToSocketAddrs>(addr: A) -> Result<TcpSocket, Error> {
        let stream = TcpStream::connect(addr)?;
        stream.set_nonblocking(true).unwrap();
        stream.set_nodelay(true).unwrap();

        let (tx, rx) = mpsc::channel();

        std::thread::spawn({
            let mut stream = stream.try_clone().unwrap();
            move || {
                let mut messages = MessageReader::new();
                loop {
                    if let Some(message) = messages.next(&mut stream) {
                        tx.send(message).unwrap();
                    }
                }
            }
        });

        Ok(TcpSocket { stream, rx })
    }
}
