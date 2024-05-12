use std::vec;

use crate::api::*;

pub type ParticleApi = crate::api::SimulationState;
pub trait Plugin {
    fn register(&mut self) -> PluginResult;
    fn update(&self, api: &mut ParticleApi);
    // Called when the simulation adds or remove a new Plugin
    // So particles can cache the id of other particles
    fn on_plugin_changed(&mut self, _: &ParticleApi) {}
}

pub struct PluginResult {
    pub name: String,
    pub color: Color,
    pub color2: Color,
}

impl Default for PluginResult {
    fn default() -> Self {
        PluginResult {
            name: String::from("Empty"),
            color: NOT_BLACK,
            color2: NOT_BLACK,
        }
    }
}

impl From<PluginResult> for ParticleCommonData {
    fn from(plugin_result: PluginResult) -> Self {

        let color = Color::from(plugin_result.color);
        let color2 = Color::from(plugin_result.color2);
        let (h,s,l) = rgb_to_hsl(color);
        let (h2, s2, l2) = rgb_to_hsl(color2);

        ParticleCommonData {
            name: plugin_result.name,
            color: plugin_result.color.into(),
            color2: plugin_result.color2.into(),
            color_hsl: [h, s, l],
            color_hsl2: [h2, s2, l2],
        }
    }
}

pub struct Empty;

impl Plugin for Empty {
    fn register(&mut self) -> PluginResult {
        PluginResult {
            name: String::from("Empty"),
            color: NOT_BLACK,
            ..Default::default()
        }
    }

    fn update(&self, _api: &mut SimulationState) {}
}

#[derive(Debug)]
pub struct ParticleCommonData {
    pub name: String,
    pub color: [u8; 4],
    pub color2: [u8; 4],
    pub color_hsl: [f32; 3],
    pub color_hsl2: [f32; 3],
}

// impl ParticleCommonData {
//     pub fn get_color(&self) -> Color {
//         Color::new(
//             self.color[0] as f32 * TO_NORMALIZED_COLOR,
//             self.color[1] as f32 * TO_NORMALIZED_COLOR,
//             self.color[2] as f32 * TO_NORMALIZED_COLOR,
//             self.color[3] as f32 * TO_NORMALIZED_COLOR,
//         )
//     }
// }

pub struct PluginData {
    pub(crate) plugins: Vec<Box<dyn Plugin>>,
    // pub(crate) libraries: Vec<libloading::Library>,
}

impl PluginData {
    pub fn notify(&mut self, api: &ParticleApi) {
        for i in 0..self.plugins.len() {
            self.plugins[i].on_plugin_changed(api);
        }
    }
}

impl Drop for PluginData {
    fn drop(&mut self) {
        // Drop first plugins, then libraries
        for plugin in self.plugins.drain(..) {
            drop(plugin);
        }

        // for library in self.libraries.drain(..) {
        //     drop(library);
        // }
    }
}

impl PluginData {
    pub fn new() -> PluginData {
        PluginData {
            // libraries: Vec::new(),
            plugins: vec![Box::new(Empty)],
        }
    }
}
