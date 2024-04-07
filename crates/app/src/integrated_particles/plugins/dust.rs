use app_core::*;
use crate::*;
use super::super::*;

pub struct Dust {
    collision_targets: [u8; 1]
}

impl Dust {
    pub fn new() -> Self {
        Dust { collision_targets: [0]}
    }
}

impl Plugin for Dust {
    fn register(&mut self) -> PluginResult {
        PluginResult {
            name: String::from("Dust"),
            color: app_core::Color::from_rgba(108,108,100,128),
            ..Default::default()
        }
    }

    fn update(&self, _: Particle, api: &mut ParticleApi) {
        let random_horizontal = api.gen_range(-1, 1);
        let down = -1;

        let _ = swap_if_match(api, random_horizontal, down, &self.collision_targets);
    }
}