use app_core::{api::Particle, Plugin};
use rand::Rng;

#[derive(Debug)]
struct Sand;

impl Plugin for Sand {
    fn register(&mut self) -> app_core::PluginResult {
        app_core::PluginResult {
            name: String::from("Sand"),
            color: 0xFFFF00,
            update_func: |cell, api| {
                // random dir between -1 and 1
                let dir = (rand::thread_rng().gen_range(0..3) - 1) as i32;

                if api.get(0, -1) == Particle::EMPTY {
                    api.set(0, -1, cell);
                    api.set(0, 0, Particle::EMPTY);
                } else if api.get(dir, -1) == Particle::EMPTY {
                    api.set(dir, -1, cell);
                    api.set(0, 0, Particle::EMPTY);
                } else if api.get(-dir, -1) == Particle::EMPTY {
                    api.set(-dir, -1, cell);
                    api.set(0, 0, Particle::EMPTY);
                }
            },
        }
    }
}

#[no_mangle]
pub fn plugin() -> Box<dyn Plugin> {
    Box::new(Sand)
}
