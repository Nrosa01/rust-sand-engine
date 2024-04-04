pub mod simulation_state;
pub mod particle;
pub mod plugin;
pub mod simulation;
pub mod order_scheme;
pub mod custom_range;

pub(crate) use crate::simulation_state::*;
pub use crate::particle::*;
pub use crate::plugin::*;
pub(crate) use crate::custom_range::*;
pub use crate::simulation::*;
pub(crate) use crate::order_scheme::*;
pub use macroquad::prelude::*;

pub const TO_NORMALIZED_COLOR: f32 = 1.0 / 255.0;
pub const FROM_NORMALIZED_TO_COLOR: f32 = 255.0;
pub const TRANSPARENT: Color = Color{ r: 0.07, g: 0.13, b: 0.17, a: 1.0 };