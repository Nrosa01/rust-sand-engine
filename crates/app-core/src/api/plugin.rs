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
    pub alpha: Vec2,
    pub extra: Vec2,
}

impl Default for PluginResult {
    fn default() -> Self {
        PluginResult {
            name: String::from("Empty"),
            color: NOT_BLACK,
            alpha: Vec2 { x: 0.9, y: 1.0 },
            extra: Vec2 { x: 0.0, y: 0.0 },
        }
    }
}

impl From<PluginResult> for ParticleCommonData {
    fn from(plugin_result: PluginResult) -> Self {

        let color = Color::from(plugin_result.color);
        let (h,s,l) = rgb_to_hsl(color);

        ParticleCommonData {
            name: plugin_result.name,
            color: plugin_result.color.into(),
            rand_alpha_min: (plugin_result.alpha.x * FROM_NORMALIZED_TO_COLOR) as u8,
            rand_alpha_max: (plugin_result.alpha.y * FROM_NORMALIZED_TO_COLOR) as u8,
            rand_extra_min: (plugin_result.extra.x * FROM_NORMALIZED_TO_COLOR) as u8,
            rand_extra_max: (plugin_result.extra.y * FROM_NORMALIZED_TO_COLOR) as u8,
            color_h: h,
            color_s: s,
            color_l: l,
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
    pub color_h: f32,
    pub color_s: f32,
    pub color_l: f32,
    pub rand_alpha_min: u8,
    pub rand_alpha_max: u8,
    pub rand_extra_min: u8,
    pub rand_extra_max: u8,
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
