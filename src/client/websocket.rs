use std::net::ToSocketAddrs;

use sapp_jsutils::JsObject;

use super::error::Error;

pub struct WebSocket;

#[no_mangle]
pub extern "C" fn quad_net_crate_version() -> u32 {
    let major = env!("CARGO_PKG_VERSION_MAJOR").parse::<u32>().unwrap();
    let minor = env!("CARGO_PKG_VERSION_MINOR").parse::<u32>().unwrap();
    let patch = env!("CARGO_PKG_VERSION_PATCH").parse::<u32>().unwrap();

    (major << 24) + (minor << 16) + patch
}

extern "C" {
    fn ws_connect(addr: JsObject);
    fn ws_send(buffer: JsObject);
    fn ws_try_recv() -> JsObject;
    fn ws_is_connected() -> i32;
}

impl WebSocket {
    pub fn send(&mut self, data: &[u8]) {
        unsafe { ws_send(JsObject::buffer(data)) };
    }

    pub fn try_recv(&mut self) -> Option<Vec<u8>> {
        let data = unsafe { ws_try_recv() };
        if data.is_nil() == false {
            let mut buf = vec![];
            data.to_byte_buffer(&mut buf);
            return Some(buf);
        }
        None
    }

    pub fn is_connected(&self) -> bool {
        unsafe { ws_is_connected() == 1 }
    }

    pub fn connect<A: ToSocketAddrs + std::fmt::Display>(addr: A) -> Result<WebSocket, Error> {
        unsafe { ws_connect(JsObject::string(&format!("{}", addr))) };

        Ok(WebSocket)
    }
}
