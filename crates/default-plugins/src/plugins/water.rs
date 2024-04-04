use app_core::*;
use crate::*;

pub struct Water;

impl Water {
    pub fn new() -> Self {
        Water
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

        let _ = move_if_empty(api, 0, dir_y) || 
                move_if_empty(api, dir_x, dir_y) || 
                move_if_empty(api, -dir_x, dir_y) || 
                move_if_empty(api, dir_x, 0) || 
                move_if_empty(api, -dir_x, 0);
    }
}
