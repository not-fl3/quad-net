use nanoserde::DeBin;
use std::sync::{Arc, Mutex};
use std::time::Duration;

#[derive(Default)]
struct ClientState {
    id: Option<usize>,
}

struct World {
    pos: (f32, f32),
    last_edit_id: usize,
    unique_id: usize,
}

pub fn main() -> std::io::Result<()> {
    let world = Arc::new(Mutex::new(World {
        pos: (100.0, 100.0),
        last_edit_id: 0,
        unique_id: 0,
    }));

    quad_net::quad_socket::server::listen(
        "0.0.0.0:8090",
        "0.0.0.0:8091",
        quad_net::quad_socket::server::Settings {
            on_message: {
                let world = world.clone();
                move |mut _out, mut state: &mut ClientState, msg| {
                    let msg: (f32, f32) = DeBin::deserialize_bin(&msg).unwrap();

                    if state.id.is_none() {
                        state.id = Some(world.lock().unwrap().unique_id);
                        world.lock().unwrap().unique_id += 1;
                    }
                    world.lock().unwrap().last_edit_id = state.id.unwrap();
                    world.lock().unwrap().pos = msg;
                }
            },
            on_timer: move |out, _state| {
                let world = world.lock().unwrap();
                out.send_bin(&(world.pos.0, world.pos.1, world.last_edit_id))
                    .unwrap();
            },
            on_disconnect: |_| {},
            timer: Some(Duration::from_millis(100)),
            _marker: std::marker::PhantomData,
        },
    );
    Ok(())
}
