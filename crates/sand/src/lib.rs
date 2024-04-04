use app_core::{api::Particle, ParticleApi, Plugin};

struct Sand {
    collision_targets: [u8; 2]
}

impl Sand {
    pub fn new() -> Self {
        Sand { collision_targets: [0,2]}
    }
}

pub fn swap_if_match(api: &mut ParticleApi, x: i32, y: i32, collision_targets: &[u8]) -> bool {
    if api.is_any_particle_at(x, y, collision_targets) {
        return api.swap(x, y);
    }
    false
}

impl Plugin for Sand {
    fn register(&mut self) -> app_core::api::ParticleCommonData {
        app_core::api::ParticleCommonData {
            name: String::from("Sand"),
            color: app_core::Color::from_hex(0xFFFF00),
        }
    }

    fn update(&self, _: Particle, api: &mut ParticleApi) {
        let random_horizontal = api.gen_range(-1, 1);
        let down = -1;

        let _ = swap_if_match(api, 0, down, &self.collision_targets) || 
                swap_if_match(api, random_horizontal, down, &self.collision_targets) || 
                swap_if_match(api, -random_horizontal, down, &self.collision_targets);
    }

    fn post_update(&mut self, api: &ParticleApi) {
        self.collision_targets[1] = api.id_from_name("Water");
    }
}

#[no_mangle]
pub fn plugin() -> Vec<Box<dyn Plugin>> {
    vec![Box::new(Sand::new())]
}
