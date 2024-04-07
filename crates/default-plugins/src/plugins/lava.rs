use app_core::*;
use crate::*;

pub struct Lava {
}

impl Lava {
    pub fn new() -> Self {
        Lava { }
    }
}

impl Plugin for Lava {
    fn register(&mut self) -> PluginResult {
        PluginResult {
            name: String::from("Lava"),
            color: app_core::Color::from_rgba(255,12,12,255),
            alpha: Vec2 { x: 1.0, y: 1.0 },
            ..Default::default()
        }
    }

    fn update(&self, _: Particle, api: &mut ParticleApi) {
        let random_horizontal = api.gen_range(-1, 1);
        let down = -1;

        let _ = try_convert(api, 0, 1, api.id_from_name("Water"), api.id_from_name("Steam")) ||
                try_convert(api, 0, -1, api.id_from_name("Water"), api.id_from_name("Rock")) ||
                move_if_empty(api, 0, down) || 
                move_if_empty(api, random_horizontal, down) || 
                move_if_empty(api, -random_horizontal, down) ||
                move_if_empty(api, random_horizontal, 0) ||
                move_if_empty(api, -random_horizontal, 0);
    }
}