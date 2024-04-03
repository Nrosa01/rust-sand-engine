use app_core::{api::Particle, ParticleApi, Plugin};

struct Sand {
    count: i32,
}

impl Plugin for Sand {
    fn register(&mut self) -> app_core::api::ParticleCommonData {
        app_core::api::ParticleCommonData {
            name: String::from("Sand"),
            color: app_core::Color::from_hex(0xFFFF00),
        }
    }

    fn update(&self, cell: Particle, api: &mut ParticleApi) {
        let dir = api.gen_range(-1,1);

        let down_direction = -1;
        // if self.count != 0 {
        //     return;
        // }

        if api.get(0, down_direction) == Particle::EMPTY {
            api.swap(0, down_direction, cell);
        } else if api.get(dir, down_direction) == Particle::EMPTY {
            api.swap(dir, down_direction, cell);
        } else if api.get(-dir, down_direction) == Particle::EMPTY {
            api.swap(-dir, down_direction, cell);
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
