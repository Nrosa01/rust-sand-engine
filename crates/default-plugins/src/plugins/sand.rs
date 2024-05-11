use app_core::*;
use crate::*;

pub struct Sand {
    collision_targets: [u8; 2]
}

impl Sand {
    pub fn new() -> Self {
        Sand { collision_targets: [0,2]}
    }
}

impl Plugin for Sand {
    fn register(&mut self) -> PluginResult {
        PluginResult {
            name: String::from("Sand"),
            color: app_core::Color::from_hex(0xFFFF00),
            color2: app_core::Color::from_hex(0xFFFF00),
        }
    }

    fn update(&self, api: &mut ParticleApi) {
        let random_horizontal = api.gen_range(-1, 1);
        let down = -1;

        let _ = swap_if_match(api, 0, down, &self.collision_targets) || 
                swap_if_match(api, random_horizontal, down, &self.collision_targets) || 
                swap_if_match(api, -random_horizontal, down, &self.collision_targets);
    }

    fn on_plugin_changed(&mut self, api: &ParticleApi) {
        self.collision_targets[1] = api.id_from_name("Water");
    }
}