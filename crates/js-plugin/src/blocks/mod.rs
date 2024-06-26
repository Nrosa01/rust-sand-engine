pub mod actions;
pub mod conditions;
pub mod numbers;
pub mod transformations;
pub mod utiliies;
pub mod direction;

pub(crate) use actions::*;
pub(crate) use conditions::*;
pub(crate) use direction::*;
pub(crate) use numbers::*;
pub(crate) use transformations::*;
pub(crate) use utiliies::*;

use app_core::{ParticleApi, Transformation};
use serde::{Deserialize, Serialize};

use crate::plugins::JSPlugin;
type Condition = Box<Conditions>;

