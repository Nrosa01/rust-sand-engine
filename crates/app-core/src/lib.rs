pub mod api;
use crate::api::{GameState, ParticleDefinition};
use macroquad::color::Color;

pub struct PluginResult {
    pub name: String,
    pub color: u32,
    pub update_func: fn(&mut GameState, usize, usize) -> (),
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
}
