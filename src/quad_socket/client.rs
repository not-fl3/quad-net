use std::net::ToSocketAddrs;

#[cfg(not(target_arch = "wasm32"))]
mod tcp;
#[cfg(target_arch = "wasm32")]
use crate::web_socket::js_web_socket as websocket;

use crate::error::Error;

pub struct QuadSocket {
    #[cfg(not(target_arch = "wasm32"))]
    tcp_socket: tcp::TcpSocket,
    #[cfg(target_arch = "wasm32")]
    web_socket: websocket::WebSocket,
}

impl QuadSocket {
    pub fn send(&mut self, data: &[u8]) -> Result<(), Error> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.tcp_socket.send(data)?;
        }

        #[cfg(target_arch = "wasm32")]
        {
            self.web_socket.send_bytes(data);
        }

        Ok(())
    }

    pub fn try_recv(&mut self) -> Option<Vec<u8>> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.tcp_socket.try_recv()
        }

        #[cfg(target_arch = "wasm32")]
        {
            self.web_socket.try_recv()
        }
    }
}

#[cfg(feature = "nanoserde")]
impl QuadSocket {
    pub fn send_bin<T: nanoserde::SerBin>(&mut self, data: &T) -> Result<(), Error>  {
        use nanoserde::SerBin;

        self.send(&SerBin::serialize_bin(data))
    }

    pub fn try_recv_bin<T: nanoserde::DeBin + std::fmt::Debug>(&mut self) -> Option<T> {
        let bytes = self.try_recv()?;
        let data: T = nanoserde::DeBin::deserialize_bin(&bytes).expect("Cant parse message");

        Some(data)
    }
}

impl QuadSocket {
    #[cfg(target_arch = "wasm32")]
    pub fn is_wasm_websocket_connected(&self) -> bool {
        self.web_socket.connected()
    }

    pub fn connect<A: ToSocketAddrs + std::fmt::Display>(addr: A) -> Result<QuadSocket, Error> {
        Ok(QuadSocket {
            #[cfg(not(target_arch = "wasm32"))]
            tcp_socket: tcp::TcpSocket::connect(addr)?,
            #[cfg(target_arch = "wasm32")]
            web_socket: websocket::WebSocket::connect(addr)?,
        })
    }
}
