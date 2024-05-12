use super::*;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "number", content = "data", rename_all = "camelCase")]
pub enum Number {
    NumberOfXTouching(Vec<Number>), // Requires ParticleType or panics
    RandomFromXToY(Box<Number>, Box<Number>),
    Opacity(Direction),
    ColorFade(Direction),
    HueShift(Direction),
    Extra(Direction),
    Extra2(Direction),
    Extra3(Direction),
    MathOperation(MathOperations, Box<Number>, Box<Number>),
    Constant(i32),

    // Particle Types
    FromID(u8), // This shouldn't be used at all, it's more an internal block
    FromName(String),
    TypeOf(Direction),
}

// Enum that holds values that cannot be precomputed
impl Number {
    pub fn to_particle_id(&self, api: &ParticleApi) -> u8 {
        match self {
            Number::FromID(id) => *id,
            Number::FromName(name) => api.id_from_name(name),
            Number::TypeOf(direction) => {
                let direction = direction.get_direction(api);
                let direction = api.get_transformation().transform(&direction);
                api.get_type(direction[0], direction[1])
            }
            _ => self.to_number(api) as u8,
        }
    }

    pub fn to_number(&self, api: &ParticleApi) -> i32 {
        match self {
            Number::NumberOfXTouching(particle_type_vec) => particle_type_vec
                .iter()
                .map(|particle_type| particle_type.to_particle_id(api))
                .flat_map(|particle_id| {
                    ParticleApi::NEIGHBORS
                        .iter()
                        .filter(move |dir| api.get_type(dir.x, dir.y) == particle_id)
                })
                .count() as i32,
            Number::RandomFromXToY(min, max) => {
                let min_number = min.to_number(api);
                let max_number = max.to_number(api);
                let min_final = min_number.min(max_number);
                let max_final = min_number.max(max_number);
                api.gen_range(min_final, max_final)
            }
            Number::Opacity(direction) => {
                let direction = direction.get_direction(api);
                let direction = api.get_transformation().transform(&direction);
                api.get(direction[0], direction[1]).opacity as i32
            }
            Number::Extra(direction) => {
                let direction = direction.get_direction(api);
                let direction = api.get_transformation().transform(&direction);
                api.get(direction[0], direction[1]).extra as i32
            }
            Number::MathOperation(op, number1, number2) => {
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
            }
            Number::Constant(constant) => *constant,
            Number::ColorFade(direction) => {
                let direction = direction.get_direction(api);
                let direction = api.get_transformation().transform(&direction);
                api.get(direction[0], direction[1]).color_fade as i32
            }
            Number::HueShift(direction) => {
                let direction = direction.get_direction(api);
                let direction = api.get_transformation().transform(&direction);
                api.get(direction[0], direction[1]).hue_shift as i32
            }
            Number::Extra2(direction) => {
                let direction = direction.get_direction(api);
                let direction = api.get_transformation().transform(&direction);
                api.get(direction[0], direction[1]).extra2 as i32
            }
            Number::Extra3(direction) => {
                let direction = direction.get_direction(api);
                let direction = api.get_transformation().transform(&direction);
                api.get(direction[0], direction[1]).extra3 as i32
            }
            _ => self.to_particle_id(api) as i32,
        }
    }
}
