//! Various network abstractions over web and desktop.

#[cfg(not(target_arch = "wasm32"))]
extern crate qws as ws;

mod error;

pub mod http_request;
pub mod quad_socket;
pub mod web_socket;

#[no_mangle]
pub extern "C" fn quad_net_crate_version() -> u32 {
	1
}
