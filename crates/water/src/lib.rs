use app_core::{api::Particle, ParticleApi, Plugin};

struct Water;

impl Plugin for Water {
    fn register(&mut self) -> app_core::PluginResult {
        app_core::PluginResult {
            name: String::from("Water"),
            color: 0x00FFFF,
        }
    }

    fn update(&self, _cell: Particle, _api: &mut ParticleApi) {
        
    }
}

#[no_mangle]
pub fn plugin() -> Box<dyn Plugin> {
    Box::new(Water)
}
