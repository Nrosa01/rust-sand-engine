pub mod simulation_state;
pub mod particle;
pub mod plugin;
pub mod simulation;
pub mod order_scheme;
pub mod custom_range;
pub mod color;
pub mod vec2;

pub(crate) use crate::simulation_state::*;
pub use crate::particle::*;
pub use crate::plugin::*;
pub(crate) use crate::custom_range::*;
pub use crate::simulation::*;
pub(crate) use crate::order_scheme::*;
pub use crate::color::*;
pub use crate::vec2::*;

pub const TO_NORMALIZED_COLOR: f32 = 1.0 / 255.0;
pub const FROM_NORMALIZED_TO_COLOR: f32 = 255.0;