use app_core::{api::Particle, ParticleApi, Plugin};

struct Sand {
    collision_targets: [u8; 2]
}

impl Sand {
    pub fn new() -> Self {
        Sand { collision_targets: [0,2]}
    }
}

impl Plugin for Sand {
    fn register(&mut self) -> app_core::api::ParticleCommonData {
        app_core::api::ParticleCommonData {
            name: String::from("Sand"),
            color: app_core::Color::from_hex(0xFFFF00),
        }
    }

    fn update(&self, cell: Particle, api: &mut ParticleApi) {
        let dir = api.gen_range(-1, 1);

        let down_direction = -1;

        if api.is_any_particle_at(0, down_direction, &self.collision_targets) {
            api.swap(0, down_direction, cell);
        } else if api.is_any_particle_at(dir, down_direction, &self.collision_targets) {
            api.swap(dir, down_direction, cell);
        } else if api.is_any_particle_at(-dir, down_direction, &self.collision_targets) {
            api.swap(-dir, down_direction, cell);
        }
    }

    fn post_update(&mut self, api: &ParticleApi) {
        self.collision_targets[1] = api.id_from_name("Water");
    }
}

#[no_mangle]
pub fn plugin() -> Vec<Box<dyn Plugin>> {
    vec![Box::new(Sand::new())]
}
