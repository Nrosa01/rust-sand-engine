use macroquad::prelude::*;
use std::vec;
use crate::api::*;

pub type ParticleApi = crate::api::SimulationState;
pub trait Plugin {
    fn register(&mut self) -> ParticleCommonData;
    fn update(&self, cell: Particle, api: &mut ParticleApi);
    fn post_update(&mut self, _: &ParticleApi) {}
}


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


pub struct Empty;

impl Plugin for Empty {
    fn register(&mut self) -> ParticleCommonData {
        ParticleCommonData {
            name: String::from("Empty"),
            color: TRANSPARENT,
        }
    }

    fn update(&self, _cell: Particle, _api: &mut SimulationState) {}
}

#[derive(Debug)]
pub struct ParticleCommonData {
    pub name: String,
    pub color: Color,
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


