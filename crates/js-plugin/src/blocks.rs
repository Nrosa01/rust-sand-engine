use app_core::{Particle, ParticleApi, Transformation};
use serde::{Deserialize, Serialize};

use crate::plugins::JSPlugin;

type Action = Box<Actions>;
type Condition = Box<Conditions>;
type ParticleType = u8;
type NumberLiteral = usize;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "direction", content = "data", rename_all = "camelCase")]
pub enum Direction {
    Constant([i32; 2]),
    Random,
}

impl Direction {
    pub fn get_direction(&self, api: &ParticleApi) -> [i32; 2] {
        match self {
            Direction::Constant(direction) => *direction,
            Direction::Random => [api.gen_range(-1, 1), api.gen_range(-1, 1)],
        }
    }
}

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "action", content = "data", rename_all = "camelCase")]
#[rustfmt::skip]
pub enum Actions
{
    Swap { direction: Direction },
    CopyTo { direction: Direction },
    ChangeInto { direction: Direction, r#type: NumbersConstant },
    RandomTransformation { transformation: TransformationInternal, block: Action},
    ForEachTransformation { transformation: TransformationInternal, block: Action},
    RotatedBy { number: Number, block: Action },
    If { condition: Conditions, result: Action, r#else: Option<Action>}, // If block, if true, execute action
    IncreaseParticlePropierty { propierty: ParticlePropierties, number: Number },
    SetParticlePropierty { propierty: ParticlePropierties, number: Number },
    Repeat { number: Number, block: Action },
    EveryXFrames { number: Number, block: Action },
}

impl Actions {
    pub fn to_func(
        &self,
        api: &ParticleApi,
    ) -> Box<dyn Fn(&JSPlugin, Particle, &mut ParticleApi) -> ()> {
        let block = self.clone();
        match block {
            Actions::Swap { direction } => match direction {
                Direction::Constant(direction) => {
                    let direction = direction;
                    Box::new(move |_, _, api| {
                        let direction = api.get_transformation().transform(&direction);
                        api.swap(direction[0], direction[1]);
                    })
                }
                _ => Box::new(move |_, _, api| {
                    let direction = direction.get_direction(api);
                    let direction = api.get_transformation().transform(&direction);
                    api.swap(direction[0], direction[1]);
                }),
            },
            Actions::CopyTo { direction } => match direction {
                Direction::Constant(direction) => {
                    let direction = direction;
                    Box::new(move |_, particle, api| {
                        let direction = api.get_transformation().transform(&direction);
                        api.set(direction[0], direction[1], particle);
                    })
                }
                _ => Box::new(move |_, particle, api| {
                    let direction = direction.get_direction(api);
                    let direction = api.get_transformation().transform(&direction);
                    api.set(direction[0], direction[1], particle);
                }),
            },
            Actions::ChangeInto { direction, r#type } => {
                let particle_id = r#type.get_particle_id(api) as u8;
                let new_particle = api.new_particle(particle_id);

                match direction {
                    Direction::Constant(direction) => {
                        let direction = direction;
                        Box::new(move |_, _, api| {
                            let direction = api.get_transformation().transform(&direction);
                            api.set(direction[0], direction[1], new_particle);
                        })
                    }
                    _ => Box::new(move |_, _, api| {
                        let direction = direction.get_direction(api);
                        let direction = api.get_transformation().transform(&direction);
                        api.set(direction[0], direction[1], new_particle);
                    }),
                }
            }

            Actions::RandomTransformation {
                transformation,
                block,
            } => {
                let func = block.to_func(api);

                Box::new(move |plugin, particle, api| {
                    let previous_trasnformation = api.get_transformation().clone();

                    let transformation = transformation.to_transformation(api);
                    api.set_transformation(transformation);

                    func(plugin, particle, api);

                    api.set_transformation(previous_trasnformation);
                })
            }
            // This is a for.. This shoould not return a bool, this should be separated into other enum
            Actions::ForEachTransformation {
                transformation,
                block,
            } => {
                let func = block.to_func(api);

                match transformation {
                    TransformationInternal::HorizontalReflection => {
                        Box::new(move |plugin, particle, api| {
                            let previous_trasnformation = api.get_transformation().clone();

                            let transformation = Transformation::HorizontalReflection(true);
                            api.set_transformation(transformation);

                            func(plugin, particle, api);

                            let transformation = Transformation::HorizontalReflection(false);
                            api.set_transformation(transformation);

                            func(plugin, particle, api);

                            api.set_transformation(previous_trasnformation);
                        })
                    }
                    TransformationInternal::VerticalReflection => {
                        Box::new(move |plugin, particle, api| {
                            let previous_trasnformation = api.get_transformation().clone();

                            let transformation = Transformation::VerticalReflection(true);
                            api.set_transformation(transformation);

                            func(plugin, particle, api);

                            let transformation = Transformation::VerticalReflection(false);
                            api.set_transformation(transformation);

                            func(plugin, particle, api);

                            api.set_transformation(previous_trasnformation);
                        })
                    }
                    TransformationInternal::Reflection => Box::new(move |plugin, particle, api| {
                        let previous_trasnformation = api.get_transformation().clone();

                        let transformation = Transformation::Reflection(true, true);
                        api.set_transformation(transformation);

                        func(plugin, particle, api);

                        let transformation = Transformation::Reflection(false, false);
                        api.set_transformation(transformation);

                        func(plugin, particle, api);

                        let transformation = Transformation::Reflection(false, true);
                        api.set_transformation(transformation);

                        func(plugin, particle, api);

                        let transformation = Transformation::Reflection(true, false);
                        api.set_transformation(transformation);

                        func(plugin, particle, api);

                        api.set_transformation(previous_trasnformation);
                    }),
                    TransformationInternal::Rotation => Box::new(move |plugin, particle, api| {
                        let previous_transformation = api.get_transformation().clone();

                        for i in 1..=7 {
                            let transformation = Transformation::Rotation(i);
                            api.set_transformation(transformation);

                            func(plugin, particle, api);
                        }

                        api.set_transformation(previous_transformation);
                    }),
                    TransformationInternal::None => {
                        Box::new(move |plugin, particle, api| func(plugin, particle, api))
                    }
                }
            }
            Actions::If {
                condition,
                result,
                r#else,
            } => {
                // We bake the functions here so they don't have to get built every time this block is called
                let condition = condition.to_func(api);
                let result = result.to_func(api);
                // "Baking" so we only call r#else if there is an else
                // I could also create an empty function with let r#else = r#else.unwrap_or_else(|| Box::new(|_, _, _| true));
                // And I could return a single lamnbda instead of this but....
                // These funcs might get called a lot, this is hotpath so I prefer to optimize as much as possible
                match r#else {
                    Some(r#else) => {
                        let r#else = r#else.to_func(api);
                        Box::new(move |plugin, particle, api| {
                            if condition(plugin, particle, api) {
                                result(plugin, particle, api)
                            } else {
                                r#else(plugin, particle, api)
                            };
                        })
                    }
                    // By default, an if blocks returns true
                    None => Box::new(move |plugin, particle, api| {
                        if condition(plugin, particle, api) {
                            result(plugin, particle, api);
                        }
                    }),
                }
            }
            Actions::RotatedBy { number, block } => {
                match number {
                    Number::NumbersRuntime(runtime) => {
                        let func = block.to_func(api);
                        Box::new(move |plugin, particle, api| {
                            let previous_transformation = api.get_transformation().clone();
                            let rotations = runtime.to_number(api);
                            // As this is a runtime number, we have to force it to be between 0 and 7 using modulo
                            let rotations = rotations % 8;
                            let transformation = Transformation::Rotation(rotations as usize);
                            api.set_transformation(transformation);
                            func(plugin, particle, api);
                            api.set_transformation(previous_transformation);
                        })
                    }
                    Number::NumbersConstant(constant) => {
                        let func = block.to_func(api);
                        let rotations = constant.get_particle_id(api);
                        let rotations = (rotations % 8) as usize; // Important to do, thankfully we can cache this
                        Box::new(move |plugin, particle, api| {
                            let previous_transformation = api.get_transformation().clone();
                            let transformation = Transformation::Rotation(rotations);
                            api.set_transformation(transformation);
                            func(plugin, particle, api);
                            api.set_transformation(previous_transformation);
                        })
                    }
                }
            }
            Actions::IncreaseParticlePropierty { propierty, number } => {
                match number {
                    Number::NumbersRuntime(number) => {
                        match propierty {
                            ParticlePropierties::Light => Box::new(move |_, particle, api| {
                                let number = number.to_number(api) as i8;
                                let mut particle = particle;
                                particle.light = particle.light.saturating_add_signed(number); // This is to avoid overflow
                                api.set(0, 0, particle);
                            }),
                            ParticlePropierties::Extra => Box::new(move |_, particle, api| {
                                let number = number.to_number(api) as i8;
                                let mut particle = particle;
                                particle.extra = particle.extra.saturating_add_signed(number); // This is to avoid overflow
                                api.set(0, 0, particle);
                            }),
                        }
                    }
                    Number::NumbersConstant(number) => {
                        let number = number.get_as_i32(api) as i8;
                        match propierty {
                            ParticlePropierties::Light => Box::new(move |_, particle, api| {
                                let mut particle = particle;
                                particle.light = particle.light.saturating_add_signed(number); // This is to avoid overflow
                                api.set(0, 0, particle);
                            }),
                            ParticlePropierties::Extra => Box::new(move |_, particle, api| {
                                let mut particle = particle;
                                particle.extra = particle.extra.saturating_add_signed(number); // This is to avoid overflow
                                api.set(0, 0, particle);
                            }),
                        }
                    }
                }
            }
            Actions::SetParticlePropierty { propierty, number } => match number {
                Number::NumbersRuntime(number) => match propierty {
                    ParticlePropierties::Light => Box::new(move |_, particle, api| {
                        let number = number.to_number(api) as u8;
                        let mut particle = particle;
                        particle.light = number.rem_euclid(u8::MAX);
                        api.set(0, 0, particle);
                    }),
                    ParticlePropierties::Extra => Box::new(move |_, particle, api| {
                        let number = number.to_number(api) as u8;
                        let mut particle = particle;
                        particle.extra = number.rem_euclid(u8::MAX);
                        api.set(0, 0, particle);
                    }),
                },
                Number::NumbersConstant(number) => {
                    let number = number.get_as_u8(api).rem_euclid(u8::MAX);
                    match propierty {
                        ParticlePropierties::Light => Box::new(move |_, particle, api| {
                            let mut particle = particle;
                            particle.light = number;
                            api.set(0, 0, particle);
                        }),
                        ParticlePropierties::Extra => Box::new(move |_, particle, api| {
                            let mut particle = particle;
                            particle.extra = number;
                            api.set(0, 0, particle);
                        }),
                    }
                }
            },
            Actions::Repeat { number, block } => {
                let func = block.to_func(api);

                match number {
                    Number::NumbersRuntime(runtime) => Box::new(move |plugin, particle, api| {
                        let times = runtime.to_number(api);
                        for _ in 0..times {
                            func(plugin, particle, api);
                        }
                    }),
                    Number::NumbersConstant(constant) => {
                        let times = constant.get_as_i32(api);
                        Box::new(move |plugin, particle, api| {
                            for _ in 0..times {
                                func(plugin, particle, api);
                            }
                        })
                    }
                }
            }
            Actions::EveryXFrames { number, block } => {
                let func = block.to_func(api);

                match number {
                    Number::NumbersRuntime(runtime) => Box::new(move |plugin, particle, api| {
                        let frames = runtime.to_number(api) as u32;
                        if api.get_frame() % frames == 0 {
                            func(plugin, particle, api);
                        }
                    }),
                    Number::NumbersConstant(constant) => {
                        let frames = constant.get_as_i32(api) as u32;
                        Box::new(move |plugin, particle, api| {
                            if api.get_frame().rem_euclid(frames) == 0 {
                                func(plugin, particle, api);
                            }
                        })
                    }
                }
            }
        }
    }
}

// Taken from Sandspiel Studio
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "block", content = "data", rename_all = "camelCase")]
#[rustfmt::skip]
pub enum Conditions {
    CheckTypeInDirection { direction: Direction, r#type: NumbersConstant }, // If particle at direction is of type X
    CheckTypesInDirection { direction: Direction, types: Vec<NumbersConstant> }, // If particle at direction is of type X
    Not { block: Condition }, // Negates a block result, it's inverting a boolean
    And { block1: Condition, block2: Condition }, // Logical AND
    Or { block1: Condition, block2: Condition }, // Logical OR
    IsTouching { r#type: NumbersConstant }, // Looks neighbour to see if it's of type X
    OneInXChance { chance: NumberLiteral }, // Returns true one in a X chance, for example, if X is 3, it will return true 1/3 of the time
    IsEmpty { direction: Direction }, // Checks if a direction is empty
    CompareNumberEquality { block1: Number, block2: Number }, // Compares two blocks
    CompareBooleans { block1: Condition, block2: Condition }, // Compares two blocks
    CompareBiggerThan { block1: Number, block2: Number }, // Compares two blocks
    CompareLessThan { block1: Number, block2: Number }, // Compares two blocks
    Boolean { value: bool }, // Returns a boolean value
}

// Implement from Block into Function
impl Conditions {
    #[allow(unused)]
    pub fn to_func(
        &self,
        api: &ParticleApi,
    ) -> Box<dyn Fn(&JSPlugin, Particle, &mut ParticleApi) -> bool> {
        let block = self.clone();
        match block {
            Conditions::CheckTypeInDirection { direction, r#type } => {
                let particle_id = r#type.get_particle_id(api) as u8;

                match direction {
                    Direction::Constant(direction) => {
                        let direction = direction;
                        Box::new(move |plugin, particle, api| {
                            let direction = api.get_transformation().transform(&direction);
                            api.get_type(direction[0], direction[1]) == particle_id
                        })
                    }
                    _ => Box::new(move |plugin, particle, api| {
                        let direction = direction.get_direction(api);
                        let direction = api.get_transformation().transform(&direction);
                        api.get_type(direction[0], direction[1]) == particle_id
                    }),
                }
            }
            Conditions::CheckTypesInDirection { direction, types } => {
                let types = types
                    .iter()
                    .map(|particle| particle.get_particle_id(api) as u8)
                    .collect::<Vec<_>>();

                // If the array is only one element, we can optimize it by taking it out.
                if types.len() == 1 {
                    let particle_id = types[0];
                    
                    match direction {
                        Direction::Constant(direction) => {
                            let direction = direction;
                            Box::new(move |plugin, particle, api| {
                                let direction = api.get_transformation().transform(&direction);
                                api.get_type(direction[0], direction[1]) == particle_id
                            })
                        }
                        _ => Box::new(move |plugin, particle, api| {
                            let direction = direction.get_direction(api);
                            let direction = api.get_transformation().transform(&direction);
                            api.get_type(direction[0], direction[1]) == particle_id
                        }),
                    }
                } else {
                    match direction {
                        Direction::Constant(direction) => {
                            let direction = direction;
                            Box::new(move |plugin, particle, api| {
                                let direction = api.get_transformation().transform(&direction);
                                api.is_any_particle_at(direction[0], direction[1], &types)
                            })
                        }
                        _ => Box::new(move |plugin, particle, api| {
                            let direction = direction.get_direction(api);
                            let direction = api.get_transformation().transform(&direction);
                            api.is_any_particle_at(direction[0], direction[1], &types)
                        }),
                    }
                }
            }
            Conditions::Not { block } => {
                let func = block.to_func(api);

                Box::new(move |plugin, particle, api| !func(plugin, particle, api))
            }
            Conditions::And { block1, block2 } => {
                let func1 = block1.to_func(api);
                let func2 = block2.to_func(api);

                Box::new(move |plugin, particle, api| {
                    func1(plugin, particle, api) && func2(plugin, particle, api)
                })
            }
            Conditions::Or { block1, block2 } => {
                let func1 = block1.to_func(api);
                let func2 = block2.to_func(api);

                Box::new(move |plugin, particle, api| {
                    func1(plugin, particle, api) || func2(plugin, particle, api)
                })
            }
            Conditions::IsTouching { r#type } => {
                let r#type = r#type.get_particle_id(api) as u8;

                Box::new(move |plugin, particle, api| {
                    ParticleApi::NEIGHBORS
                        .iter()
                        .any(|(direction)| api.get_type(direction.x, direction.y) == r#type)
                })
            }
            Conditions::CompareNumberEquality { block1, block2 } => {
                // I know this might look ugly but this is what peak performance looks like
                // As number can be runtime or constant, I have to try all the combinations
                // Sure I could just compute everything at runtime but the most function calls I can avoid the better

                match (block1, block2) {
                    (Number::NumbersRuntime(runtime1), Number::NumbersRuntime(runtime2)) => {
                        Box::new(move |plugin, particle, api| {
                            runtime1.to_number(api) == runtime2.to_number(api)
                        })
                    }
                    (Number::NumbersRuntime(runtime1), Number::NumbersConstant(constant2)) => {
                        let number2 = constant2.get_as_i32(api);
                        Box::new(move |plugin, particle, api| runtime1.to_number(api) == number2)
                    }
                    (Number::NumbersConstant(constant1), Number::NumbersRuntime(runtime2)) => {
                        let number1 = constant1.get_as_i32(api);
                        Box::new(move |plugin, particle, api| number1 == runtime2.to_number(api))
                    }
                    (Number::NumbersConstant(constant1), Number::NumbersConstant(constant2)) => {
                        let number1 = constant1.get_as_i32(api);
                        let number2 = constant2.get_as_i32(api);
                        let result = number1 == number2;
                        Box::new(move |plugin, particle, api| result)
                    }
                }
            }
            Conditions::CompareBiggerThan { block1, block2 } => match (block1, block2) {
                (Number::NumbersRuntime(runtime1), Number::NumbersRuntime(runtime2)) => {
                    Box::new(move |plugin, particle, api| {
                        runtime1.to_number(api) > runtime2.to_number(api)
                    })
                }
                (Number::NumbersRuntime(runtime1), Number::NumbersConstant(constant2)) => {
                    let number2 = constant2.get_as_i32(api);
                    Box::new(move |plugin, particle, api| runtime1.to_number(api) > number2)
                }
                (Number::NumbersConstant(constant1), Number::NumbersRuntime(runtime2)) => {
                    let number1 = constant1.get_as_i32(api);
                    Box::new(move |plugin, particle, api| number1 > runtime2.to_number(api))
                }
                (Number::NumbersConstant(constant1), Number::NumbersConstant(constant2)) => {
                    let number1 = constant1.get_as_i32(api);
                    let number2 = constant2.get_as_i32(api);
                    let result = number1 > number2;
                    Box::new(move |plugin, particle, api| result)
                }
            },
            Conditions::CompareLessThan { block1, block2 } => match (block1, block2) {
                (Number::NumbersRuntime(runtime1), Number::NumbersRuntime(runtime2)) => {
                    Box::new(move |plugin, particle, api| {
                        runtime1.to_number(api) < runtime2.to_number(api)
                    })
                }
                (Number::NumbersRuntime(runtime1), Number::NumbersConstant(constant2)) => {
                    let number2 = constant2.get_as_i32(api);
                    Box::new(move |plugin, particle, api| runtime1.to_number(api) < number2)
                }
                (Number::NumbersConstant(constant1), Number::NumbersRuntime(runtime2)) => {
                    let number1 = constant1.get_as_i32(api);
                    Box::new(move |plugin, particle, api| number1 < runtime2.to_number(api))
                }
                (Number::NumbersConstant(constant1), Number::NumbersConstant(constant2)) => {
                    let number1 = constant1.get_as_i32(api);
                    let number2 = constant2.get_as_i32(api);
                    let result = number1 < number2;
                    Box::new(move |plugin, particle, api| result)
                }
            },
            Conditions::Boolean { value } => Box::new(move |plugin, particle, api| value),
            Conditions::IsEmpty { direction } => {
                match direction {
                    Direction::Constant(direction) => {
                        let direction = direction;
                        Box::new(move |_, _, api| {
                            let direction = api.get_transformation().transform(&direction);
                            api.is_empty(direction[0], direction[1])
                        })
                    }
                    _ => Box::new(move |_, _, api| {
                        let direction = direction.get_direction(api);
                        let direction = api.get_transformation().transform(&direction);
                        api.is_empty(direction[0], direction[1])
                    }),
                }
            }
            Conditions::OneInXChance { chance } => {
                let chance = chance as i32;
                Box::new(move |plugin, particle, api| {
                    let random_number = api.gen_range(1, chance);
                    random_number == 0
                })
            }
            Conditions::CompareBooleans { block1, block2 } => {
                let func1 = block1.to_func(api);
                let func2 = block2.to_func(api);

                Box::new(move |plugin, particle, api| {
                    func1(plugin, particle, api) == func2(plugin, particle, api)
                })
            }
        }
    }
}
