use super::*;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "number", content = "data", rename_all = "camelCase")]
pub enum Number {
    NumberOfXTouching(Vec<ParticleType>), // Requires ParticleType or panics
    TypeOf(Direction),
    RandomFromXToY(i32, i32),
    Light(Option<Direction>),
    Extra(Option<Direction>),
    MathOperation(MathOperations, Box<Number>, Box<Number>),
    Constant(i32),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "particle_type", content = "data", rename_all = "camelCase")]
pub enum ParticleType{
    FromID(u8), // This shouldn't be used at all, it's more an internal block
    FromName(String),
}

impl ParticleType {
    pub fn get_particle_id(&self, api: &ParticleApi) -> u8 {
        match self {
            ParticleType::FromID(id) => *id,
            ParticleType::FromName(name) => api.id_from_name(name),
        }
    }
}

#[rustfmt::skip]
// Enum that holds values that cannot be precomputed
impl Number {
    pub fn to_number(&self, api: &ParticleApi) -> i32 {
        match self {
            Number::NumberOfXTouching(particle_type_vec) => {
                particle_type_vec
                    .iter()
                    .map(|particle_type| particle_type.get_particle_id(api))
                    .flat_map(|particle_id| {
                        ParticleApi::NEIGHBORS
                            .iter()
                            .filter(move |dir| api.get_type(dir.x, dir.y) == particle_id)
                    })
                    .count() as i32
            }
            Number::TypeOf(direction) => {
                let direction = direction.get_direction(api);
                let direction = api.get_transformation().transform(&direction);
                api.get_type(direction[0], direction[1]) as i32
            },
            Number::RandomFromXToY(min, max) => api.gen_range(*min, *max),
            Number::Light(direction) => {
                let direction = direction.as_ref().unwrap_or(&Direction::Constant([0,0])).get_direction(api);
                let direction = api.get_transformation().transform(&direction);
                api.get(direction[0], direction[1]).light as i32},
            Number::Extra(direction) => {
                let direction = direction.as_ref().unwrap_or(&Direction::Constant([0,0])).get_direction(api);
                let direction = api.get_transformation().transform(&direction);
                api.get(direction[0], direction[1]).extra as i32},
            Number::MathOperation(op, number1, number2) => 
            {
                let number1 = number1.to_number(api);
                let number2 = number2.to_number(api);
                match op {
                    MathOperations::Addition => number1 + number2,
                    MathOperations::Subtraction => number1 - number2,
                    MathOperations::Multiplication => number1 * number2,
                    MathOperations::Division => number1 / number2,
                    MathOperations::Modulo => number1 % number2,
                    MathOperations::Difference => (number1 - number2).abs(),
                }
            },
            Number::Constant(constant) => *constant,
        }
    }
}
