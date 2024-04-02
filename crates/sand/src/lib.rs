use app_core::{api::GameState, Plugin};

#[derive(Debug)]
struct Sand;

impl Plugin for Sand {
    fn register(&mut self) -> app_core::PluginResult {
        app_core::PluginResult {
            name: String::from("Sand"),
            color: 0xFFFF00,
            update_func: |state, x, y| {
                if y < state.height - 1 {
                    if state.get_particle_id(x, y + 1) == 0 {
                        state.set_particle(x, y, 0);
                        state.set_particle(x, y + 1, 1);
                    }
                }
            },
        }
    }
}

#[no_mangle]
pub fn plugin() -> Box<dyn Plugin> {
    Box::new(Sand)
}
