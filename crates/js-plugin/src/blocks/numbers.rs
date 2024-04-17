use super::*;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "variadic_number", content = "data", rename_all = "camelCase")]
pub enum Number {
    NumbersRuntime(NumbersRuntime),
    NumbersConstant(NumbersConstant),
}

impl Number {
    // This should only be called at runtime by math operations in number runtime
    pub fn to_number_runtime(&self, api: &ParticleApi) -> i32 {
        match self {
            Number::NumbersRuntime(runtime) => runtime.to_number(api),
            Number::NumbersConstant(constant) => constant.get_as_i32(api),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "runtime_number", content = "data", rename_all = "camelCase")]
pub enum NumbersRuntime {
    NumberOfXTouching(NumbersConstant),
    TypeOf(Direction),
    RandomFromXToY(i32, i32),
    Light(Direction),
    Extra(Direction),
    MathOperation(MathOperations, Box<Number>, Box<Number>),
}

#[rustfmt::skip]
// Enum that holds values that cannot be precomputed
impl NumbersRuntime {
    pub fn to_number(&self, api: &ParticleApi) -> i32 {
        match self {
            NumbersRuntime::NumberOfXTouching(particle_type) => {
                let particle_type = particle_type.get_particle_id(api);
                ParticleApi::NEIGHBORS
                    .iter()
                    .filter(|dir| api.get_type(dir.x, dir.y) == particle_type)
                    .count() as i32
            }
            NumbersRuntime::TypeOf(direction) => {
                let direction = direction.get_direction(api);
                let direction = api.get_transformation().transform(&direction);
                api.get_type(direction[0], direction[1]) as i32
            },
            NumbersRuntime::RandomFromXToY(min, max) => api.gen_range(*min, *max),
            NumbersRuntime::Light(direction) => {
                let direction = direction.get_direction(api);
                let direction = api.get_transformation().transform(&direction);
                api.get(direction[0], direction[1]).light as i32},
            NumbersRuntime::Extra(direction) => {
                let direction = direction.get_direction(api);
                let direction = api.get_transformation().transform(&direction);
                api.get(direction[0], direction[1]).extra as i32},
            NumbersRuntime::MathOperation(op, number1, number2) => 
            {
                let number1 = number1.to_number_runtime(api);
                let number2 = number2.to_number_runtime(api);
                match op {
                    MathOperations::Addition => number1 + number2,
                    MathOperations::Subtraction => number1 - number2,
                    MathOperations::Multiplication => number1 * number2,
                    MathOperations::Division => number1 / number2,
                    MathOperations::Modulo => number1 % number2,
                    MathOperations::Difference => (number1 - number2).abs(),
                }
            
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "constant_number", content = "data", rename_all = "camelCase")]
pub enum NumbersConstant {
    ParticleType(ParticleType), // This shouldn't be used at all, it's more an internal block
    ParticleIdFromName(String),
    Constant(i32),
}

#[rustfmt::skip]
impl NumbersConstant {
    pub fn get_particle_id(&self, api: &ParticleApi) -> u8 {
        match self {
            NumbersConstant::ParticleType(particle_type) => *particle_type,
            NumbersConstant::ParticleIdFromName(name) => api.id_from_name(&name),
            NumbersConstant::Constant(_) => panic!("This should not be called, this is a constant number"),
        }
    } 

    pub fn get_constant(&self, _: &ParticleApi) -> i32 {
        match self {
            NumbersConstant::ParticleType(_) => panic!("This should not be called, this is a particle type"),
            NumbersConstant::ParticleIdFromName(_) => panic!("This should not be called, this is a particle name"),
            NumbersConstant::Constant(constant) => *constant,
        }
    }

    pub fn get_as_u8(&self, api: &ParticleApi) -> u8 {
        match self {
            NumbersConstant::ParticleType(particle_type) => *particle_type,
            NumbersConstant::ParticleIdFromName(name) => api.id_from_name(&name),
            NumbersConstant::Constant(constant) => *constant as u8,
        }
    }

    pub fn get_as_i32(&self, api: &ParticleApi) -> i32 {
        match self {
            NumbersConstant::ParticleType(particle_type) => *particle_type as i32,
            NumbersConstant::ParticleIdFromName(name) => api.id_from_name(&name) as i32, 
            NumbersConstant::Constant(constant) => *constant,
        }
    }
}
