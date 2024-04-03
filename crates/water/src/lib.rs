use app_core::Plugin;

struct Water;

impl Plugin for Water {
    fn register(&mut self) -> app_core::PluginResult {
        app_core::PluginResult {
            name: String::from("Water"),
            color: 0x00FFFF,
            update_func: |_cell, _api| {
                // if api.get(0, -1) == Particle::Empty {
                //     api.set(0, -1, cell);
                //     api.set(0, 0, Particle::Empty);
                // } else if api.get(-1, -1) == Particle::Empty {
                //     api.set(-1, -1, cell);
                //     api.set(0, 0, Particle::Empty);
                // } else if api.get(1, -1) == Particle::Empty {
                //     api.set(1, -1, cell);
                //     api.set(0, 0, Particle::Empty);
                // } else if api.get(-1, 0) == Particle::Empty {
                //     api.set(-1, 0, cell);
                //     api.set(0, 0, Particle::Empty);
                // } else if api.get(1, 0) == Particle::Empty {
                //     api.set(1, 0, cell);
                //     api.set(0, 0, Particle::Empty);
                // }
            },
        }
    }

    fn update(&self, _cell: app_core::api::Particle, _api: &mut app_core::api::GameState) {
        
    }
}

#[no_mangle]
pub fn plugin() -> Box<dyn Plugin> {
    Box::new(Water)
}
