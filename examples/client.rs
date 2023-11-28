use macroquad::prelude::*;

use quad_net::quad_socket::client::QuadSocket;

#[macroquad::main("Networking!")]
async fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    let mut socket = QuadSocket::connect("localhost:8090").unwrap();
    #[cfg(target_arch = "wasm32")]
    let mut socket = QuadSocket::connect("ws://localhost:8091").unwrap();

    #[cfg(target_arch = "wasm32")]
    {
        while socket.is_wasm_websocket_connected() == false {
            next_frame().await;
        }
    }

    let mut pos = vec2(0.0, 0.0);
    let mut last_edit_id = 0;

    loop {
        while let Some((x, y, id)) = socket.try_recv_bin() {
            pos.x = x;
            pos.y = y;
            last_edit_id = id;
        }
        draw_text(
            &format!("Last edited by: {}", last_edit_id),
            pos.x - 50.0,
            pos.y - 80.0,
            30.0,
            WHITE,
        );
        draw_circle(pos.x, pos.y, 15., RED);

        if is_mouse_button_down(MouseButton::Left) {
            let (x, y) = mouse_position();
            socket.send_bin(&(x, y));
        }
        next_frame().await
    }
}
