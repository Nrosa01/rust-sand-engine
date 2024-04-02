use std::borrow::Borrow;

use app_core::{
    api::{GameState, Particle},
    Plugin,
};

#[derive(Debug)]
struct Sand;

impl Plugin for Sand {
    fn register(&mut self) -> app_core::PluginResult {
        app_core::PluginResult {
            name: String::from("Sand"),
            color: 0xFFFF00,
            update_func: |cell, api| {
                let non_mutable_particle: Particle = cell.clone();
                
                if *api.get(0, -1) == Particle::EMPTY {
                    api.set(0, -1, non_mutable_particle);
                    api.set(0, 0, Particle::EMPTY);
                } else if *api.get(-1, -1) == Particle::EMPTY {
                    api.set(-1, -1, non_mutable_particle);
                    api.set(0, 0, Particle::EMPTY);
                } else if *api.get(1, -1) == Particle::EMPTY {
                    api.set(1, -1, non_mutable_particle);
                    api.set(0, 0, Particle::EMPTY);
                }
            },
        }
    }
}

#[no_mangle]
pub fn plugin() -> Box<dyn Plugin> {
    Box::new(Sand)
}
