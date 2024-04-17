use super::*;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(
    tag = "particle_propierty_descriptor",
    content = "data",
    rename_all = "camelCase"
)]
pub enum ParticlePropierties {
    Light,
    Extra,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "math_op", content = "data", rename_all = "camelCase")]
pub enum MathOperations {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Modulo,
    Difference, // Absolute difference, abs(abs(a) - abs(b))
}