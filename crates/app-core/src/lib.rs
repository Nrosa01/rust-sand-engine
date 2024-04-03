pub mod api;
use crate::api::{GameState, ParticleDefinition};
use api::Particle;
use macroquad::color::Color;

pub struct PluginResult {
    pub name: String,
    pub color: u32,
    pub update_func: fn(Particle, &mut GameState) -> (),
}

impl From<PluginResult> for ParticleDefinition {
    fn from(plugin_result: PluginResult) -> Self {
        ParticleDefinition {
            name: plugin_result.name,
            update_func: plugin_result.update_func,
            color: Color::from_hex(plugin_result.color)
        }
    }
}

pub trait Plugin {
    fn register(&mut self) -> PluginResult;
    fn update(&self, cell: Particle, api: &mut GameState);
    fn post_update(&mut self, api: &mut GameState) {}
}
