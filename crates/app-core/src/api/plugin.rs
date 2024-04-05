use std::vec;
use crate::api::*;

pub type ParticleApi = crate::api::SimulationState;
pub trait Plugin {
    fn register(&mut self) -> PluginResult;
    fn update(&self, cell: Particle, api: &mut ParticleApi);
    fn post_update(&mut self, _: &ParticleApi) {}
}

pub struct PluginResult {
    pub name: String,
    pub color: Color,
    pub alpha: Vec2,
    pub extra: Vec2,
}

impl Default for PluginResult {
    fn default() -> Self {
        PluginResult {
            name: String::from("Empty"),
            color: TRANSPARENT,
            alpha: Vec2 { x: 0.9, y: 1.0 },
            extra: Vec2 { x: 0.0, y: 0.0 },
        }
    }
}

impl From<PluginResult> for ParticleCommonData {
    fn from(plugin_result: PluginResult) -> Self {
        ParticleCommonData {
            name: plugin_result.name,
            color: plugin_result.color,
            rand_alpha_min: (plugin_result.alpha.x * FROM_NORMALIZED_TO_COLOR) as u8,
            rand_alpha_max: (plugin_result.alpha.y * FROM_NORMALIZED_TO_COLOR) as u8,
            rand_extra_min: (plugin_result.extra.x * FROM_NORMALIZED_TO_COLOR) as u8,
            rand_extra_max: (plugin_result.extra.y * FROM_NORMALIZED_TO_COLOR) as u8,
        }
    }
}


pub struct Empty;

impl Plugin for Empty {
    fn register(&mut self) -> PluginResult {
        PluginResult {
            name: String::from("Empty"),
            color: TRANSPARENT,
            ..Default::default()
        }
    }

    fn update(&self, _cell: Particle, _api: &mut SimulationState) {}
}

#[derive(Debug)]
pub struct ParticleCommonData {
    pub name: String,
    pub color: Color,
    pub rand_alpha_min: u8,
    pub rand_alpha_max: u8,
    pub rand_extra_min: u8,
    pub rand_extra_max: u8,
}

pub struct PluginData {
    pub(crate) plugins: Vec<Box<dyn Plugin>>,
    pub(crate) libraries: Vec<libloading::Library>,
}

impl Drop for PluginData {
    fn drop(&mut self) {
        // Drop first plugins, then libraries
        for plugin in self.plugins.drain(..) {
            drop(plugin);
        }

        for library in self.libraries.drain(..) {
            drop(library);
        }
    }
}

impl PluginData {
    pub fn new() -> PluginData {
        PluginData {
            libraries: Vec::new(),
            plugins: vec![Box::new(Empty)],
        }
    }
}


