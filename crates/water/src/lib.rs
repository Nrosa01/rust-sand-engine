use app_core::{api::Particle, ParticleApi, Plugin};

fn move_to(x: i32, y: i32, cell: Particle, api: &mut ParticleApi) {
    api.set(x, y, cell);
    api.set(0, 0, Particle::EMPTY);
}

struct Water;

impl Plugin for Water {
    fn register(&mut self) -> app_core::api::ParticleCommonData {
        app_core::api::ParticleCommonData {
            name: String::from("Water"),
            color: app_core::Color::from_hex(0x00FFFF),
        }
    }

    fn update(&self, cell: Particle, api: &mut ParticleApi) {
        let dir_x = api.gen_range(-1, 1);
        let dir_y = -1;

        if api.get(0, dir_y) == Particle::EMPTY {
            move_to(0, dir_y, cell, api);
        } else if api.get(dir_x, dir_y) == Particle::EMPTY {
            api.set(dir_x, dir_y, cell);
            api.set(0, 0, Particle::EMPTY);
        } else if api.get(-dir_x, dir_y) == Particle::EMPTY {
            api.set(-dir_x, dir_y, cell);
            api.set(0, 0, Particle::EMPTY);
        } else if api.get(dir_x, 0) == Particle::EMPTY {
            api.set(dir_x, 0, cell);
            api.set(0, 0, Particle::EMPTY);
        } else if api.get(-dir_x, 0) == Particle::EMPTY {
            api.set(-dir_x, 0, cell);
            api.set(0, 0, Particle::EMPTY);
        }
    }
}

#[no_mangle]
pub fn plugin() -> Box<dyn Plugin> {
    Box::new(Water)
}
