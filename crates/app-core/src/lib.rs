pub mod api;
use crate::api::{SimulationState, ParticleCommonData};
use api::Particle;
use macroquad::color::Color;


pub type ParticleApi = SimulationState;

pub struct PluginResult {
    pub name: String,
    pub color: u32,
}

impl From<PluginResult> for ParticleCommonData {
    fn from(plugin_result: PluginResult) -> Self {
        ParticleCommonData {
            name: plugin_result.name,
            color: Color::from_hex(plugin_result.color)
        }
    }
}

pub trait Plugin {
    fn register(&mut self) -> PluginResult;
    fn update(&self, cell: Particle, api: &mut ParticleApi);
    fn post_update(&mut self, _: &mut ParticleApi) {}
}
