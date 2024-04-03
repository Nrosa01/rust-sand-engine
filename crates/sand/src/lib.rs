use app_core::{api::Particle, Plugin, api::GameState};
use rand::Rng;

struct Sand {
    count: i32,
}

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

    fn update(&self, cell: Particle, api: &mut GameState) {
        let dir = (rand::thread_rng().gen_range(0..3) - 1) as i32;
        // if self.count != 0 {
        //     return;
        // }

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
    }

    fn post_update(&mut self, _api: &mut app_core::api::GameState) {
        self.count += 1;

        if self.count > 10 {
            self.count = 0;
        }
    }
}

#[no_mangle]
pub fn plugin() -> Box<dyn Plugin> {
    Box::new(Sand{count: 0})
}
