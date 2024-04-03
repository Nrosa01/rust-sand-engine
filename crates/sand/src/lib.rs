use app_core::{api::Particle, ParticleApi, Plugin};

struct Sand {
    count: i32,
}

impl Plugin for Sand {
    fn register(&mut self) -> app_core::PluginResult {
        app_core::PluginResult {
            name: String::from("Sand"),
            color: 0xFFFF00,
        }
    }

    fn update(&self, cell: Particle, api: &mut ParticleApi) {
        let dir = api.gen_range(-1,1);

        let down_direction = -1;
        // if self.count != 0 {
        //     return;
        // }

        if api.get(0, down_direction) == Particle::EMPTY {
            api.set(0, down_direction, cell);
            api.set(0, 0, Particle::EMPTY);
        } else if api.get(dir, down_direction) == Particle::EMPTY {
            api.set(dir, -1, cell);
            api.set(0, 0, Particle::EMPTY);
        } else if api.get(-dir, down_direction) == Particle::EMPTY {
            api.set(-dir, -1, cell);
            api.set(0, 0, Particle::EMPTY);
        }
    }

    fn post_update(&mut self, _: &ParticleApi) {
        self.count += 1;

        if self.count > 10 {
            self.count = 0;
        }
    }
}

#[no_mangle]
pub fn plugin() -> Box<dyn Plugin> {
    Box::new(Sand { count: 0 })
}
