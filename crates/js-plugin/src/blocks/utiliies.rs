use super::*;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(
    rename_all = "camelCase"
)]
pub enum ParticlePropierties {
    Opacity,
    HueShift,
    Extra,
    Extra2,
    Extra3,
    Extra4,
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