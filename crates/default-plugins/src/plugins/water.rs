use app_core::*;
use crate::*;

pub struct Water{
    collision_targets: [u8; 2]
}

impl Water {
    pub fn new() -> Self {
        Water  { collision_targets: [0,2] }
    }
}

impl Plugin for Water {
    fn register(&mut self) -> ParticleCommonData {
        ParticleCommonData {
            name: String::from("Water"),
            color: app_core::Color::from_hex(0x00FFFF),
        }
    }

    fn update(&self, _: Particle, api: &mut ParticleApi) {
        let dir_x = api.gen_range(-1, 1);
        let dir_y = -1;

        let _ = swap_if_match(api, 0, dir_y, &self.collision_targets) || 
                swap_if_match(api, dir_x, dir_y, &self.collision_targets) || 
                swap_if_match(api, -dir_x, dir_y, &self.collision_targets) || 
                swap_if_match(api, dir_x, 0, &self.collision_targets) || 
                swap_if_match(api, -dir_x, 0, &self.collision_targets);
    }

    fn post_update(&mut self, api: &ParticleApi) {
        self.collision_targets[1] = api.id_from_name("Dust");
    }
}
