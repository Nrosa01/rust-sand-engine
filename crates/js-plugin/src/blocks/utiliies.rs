use super::*;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(
    rename_all = "camelCase"
)]
pub enum ParticlePropierties {
    Light,
    Extra,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum MathOperations {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Modulo,
    Difference, // Absolute difference, abs(abs(a) - abs(b))
}