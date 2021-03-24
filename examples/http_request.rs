use macroquad::prelude::*;

use quad_net::http_request::RequestBuilder;

#[macroquad::main("Http request demo")]
async fn main() {
    let mut request = RequestBuilder::new("http://127.0.0.1:4000/").send();

    loop {
        if let Some(data) = request.try_recv() {
            info!("Done! {}", data.unwrap());
        }
        next_frame().await;
    }
}
