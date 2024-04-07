use app_core::*;

pub struct Rock {
}

impl Rock {
    pub fn new() -> Self {
        Rock { }
    }
}

impl Plugin for Rock {
    fn register(&mut self) -> PluginResult {
        PluginResult {
            name: String::from("Rock"),
            color: app_core::Color::from_rgba(123,133,145,255),
            ..Default::default()
        }
    }

    fn update(&self, _: Particle, _: &mut ParticleApi) {}
}