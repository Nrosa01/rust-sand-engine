use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TransformationInternal {
    HorizontalReflection,
    VerticalReflection,
    Reflection,
    Rotation,
    None,
}

impl TransformationInternal {
    pub fn to_transformation(&self, api: &ParticleApi) -> Transformation {
        match self {
            TransformationInternal::HorizontalReflection => {
                Transformation::HorizontalReflection(api.random_bool())
            }
            TransformationInternal::VerticalReflection => {
                Transformation::VerticalReflection(api.random_bool())
            }
            TransformationInternal::Reflection => {
                Transformation::Reflection(api.random_bool(), api.random_bool())
            }
            TransformationInternal::Rotation => {
                Transformation::Rotation(api.gen_range(0, 7) as usize)
            }
            TransformationInternal::None => Transformation::None,
        }
    }
}