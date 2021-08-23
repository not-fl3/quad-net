use std::net::ToSocketAddrs;

use std::net::TcpStream;
use std::sync::mpsc::{self, Receiver, SendError};

use crate::{error::Error, quad_socket::protocol::MessageReader};

pub struct TcpSocket {
    stream: TcpStream,
    rx: Receiver<Vec<u8>>,
}

impl TcpSocket {
    pub fn send(&mut self, data: &[u8]) -> Result<(), Error> {
        write_until_done(&mut self.stream, &u32::to_be_bytes(data.len() as u32))?;
        write_until_done(&mut self.stream, data)?;

        Ok(())
    }

    pub fn try_recv(&mut self) -> Option<Vec<u8>> {
        self.rx.try_recv().ok()
    }
}

fn write_until_done(stream: &mut TcpStream, message: &[u8]) -> Result<(), Error> {
    use std::io::Write;
    let mut sent = 0;

    while sent < message.len() {
        sent += stream.write(&message[sent..])
            .map_err(Error::IOError)?;
    }

    Ok(())
}

impl TcpSocket {
    pub fn connect<A: ToSocketAddrs>(addr: A) -> Result<TcpSocket, Error> {
        let stream = TcpStream::connect(addr)?;
        stream.set_nodelay(true).unwrap();
        stream.set_nonblocking(true).unwrap();

        let (tx, rx) = mpsc::channel();

        std::thread::spawn({
            let mut stream = stream.try_clone().unwrap();
            move || {
                let mut messages = MessageReader::new();
                loop {
                    match messages.next(&mut stream) {
                        Ok(Some(message)) => {
                            match tx.send(message) {
                                Ok(()) => (),
                                Err(SendError(_message)) => break,
                            }
                        }
                        Ok(None) => { std::thread::yield_now() },
                        Err(()) => break,
                    }
                }
            }
        });

        Ok(TcpSocket { stream, rx })
    }
}
