use app_core::{Particle, ParticleApi, Transformation};
use serde::{Deserialize, Serialize};

use crate::plugins::JSPlugin;

// type Action = Box<dyn Fn(Particle, &mut ParticleApi)>;
type Action = Box<Actions>;
type Condition = Box<Conditions>;
type Direction = [i32; 2];
type ParticleType = u8;
type NumberLiteral = usize;

// Right now we assume users now the ID of the particles they want to check against
// Once we get this working, we are going to transform de json and substitute
// The particles names with an index that will be indexed into an array that has the particle IDs.

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "variadic_number", content = "data", rename_all = "camelCase")]
pub enum Number {
    NumbersRuntime(NumbersRuntime),
    NumberConstants(NumberConstants),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "runtime_number", content = "data", rename_all = "camelCase")]
pub enum NumbersRuntime {
    NumberOfXTouching(NumberConstants),
    TypeOf(Direction),
}

#[rustfmt::skip]
// Enum that holds values that cannot be precomputed
impl NumbersRuntime {
    pub fn to_number(&self, api: &ParticleApi) -> usize {
        match self {
            NumbersRuntime::NumberOfXTouching(particle_type) => {
                let particle_type = particle_type.to_number(api) as u8;
                ParticleApi::NEIGHBORS
                    .iter()
                    .filter(|dir| api.get_type(dir.x, dir.y) == particle_type)
                    .count()
            }
            NumbersRuntime::TypeOf(direction) => api.get_type(direction[0], direction[1]) as usize,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "constant_number", content = "data", rename_all = "camelCase")]
pub enum NumberConstants {
    ParticleType(ParticleType), // This shouldn't be used at all, it's more an internal block
    ParticleIdFromName(String),
    Number(NumberLiteral),
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
impl NumberConstants {
    pub fn to_number(&self, api: &ParticleApi) -> usize {
        match self {
            NumberConstants::ParticleType(particle_type) => *particle_type as usize,
            NumberConstants::ParticleIdFromName(name) => api.id_from_name(&name) as usize,
            NumberConstants::Number(number) => *number,	
        }
    } 
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "action", content = "data", rename_all = "camelCase")]
#[rustfmt::skip]
pub enum Actions
{
    RandomTransformation { transformation: TransformationInternal, block: Action},
    ForEachTransformation { transformation: TransformationInternal, block: Action},
    Swap { direction: Direction },
    CopyTo { direction: Direction },
    ChangeInto { direction: Direction, r#type: NumberConstants },
    If { condition: Conditions, result: Action, r#else: Option<Action>}, // If block, if true, execute action
}

impl Actions{
    pub fn to_func(
        &self,
        api: &ParticleApi,
    ) -> Box<dyn Fn(&JSPlugin, Particle, &mut ParticleApi) -> ()>
    {
        let block = self.clone();
        match block
        {
            Actions::Swap { direction } => Box::new(move |_, _, api| {
                let direction = api.get_transformation().transform(&direction);
                api.swap(direction[0], direction[1]);
            }),
            Actions::CopyTo { direction } => Box::new(move |_, particle, api| {
                let direction = api.get_transformation().transform(&direction);
                api.set(direction[0], direction[1], particle);
            }),
            Actions::ChangeInto { direction, r#type } => {
                let particle_id = r#type.to_number(api) as u8;

                Box::new(move |_, _: Particle, api| {
                    let direction = api.get_transformation().transform(&direction);
                    api.set(direction[0], direction[1], api.new_particle(particle_id));
                })
            },
            
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
            } => 
            {
                let func = block.to_func(api);

                match transformation {
                    TransformationInternal::HorizontalReflection => 
                    {
                        Box::new(move |plugin, particle, api| 
                        {
                            let previous_trasnformation = api.get_transformation().clone();
                            
                            let transformation = Transformation::HorizontalReflection(true);
                            api.set_transformation(transformation);
                            
                            func(plugin, particle, api);
                            
                            let transformation = Transformation::HorizontalReflection(false);
                            api.set_transformation(transformation);
                            
                            func(plugin, particle, api);
                            
                            api.set_transformation(previous_trasnformation);
                        })
                    
                    },
                    TransformationInternal::VerticalReflection => 
                    {
                        Box::new(move |plugin, particle, api| 
                        {
                            let previous_trasnformation = api.get_transformation().clone();
                            
                            let transformation = Transformation::VerticalReflection(true);
                            api.set_transformation(transformation);
                            
                            func(plugin, particle, api);
                            
                            let transformation = Transformation::VerticalReflection(false);
                            api.set_transformation(transformation);
                            
                            func(plugin, particle, api);
                            
                            api.set_transformation(previous_trasnformation);
                        })
                    },
                    TransformationInternal::Reflection =>
                    {
                        Box::new(move |plugin, particle, api| 
                        {
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
                        })
                    },
                    TransformationInternal::Rotation => 
                    {
                        Box::new(move |plugin, particle, api| 
                        {
                            let previous_transformation = api.get_transformation().clone();
                            
                            for i in 1..=7 {
                                let transformation = Transformation::Rotation(i);
                                api.set_transformation(transformation);
                                
                                func(plugin, particle, api);
                            }
                            
                            api.set_transformation(previous_transformation);
                        })
                    
                    },
                    TransformationInternal::None => Box::new(move |plugin, particle, api| func(plugin, particle, api)),
                }
            },
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
        }
    }
}

// Taken from Sandspiel Studio
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "block", content = "data", rename_all = "camelCase")]
#[rustfmt::skip]
pub enum Conditions {
    CheckTypeInDirection { direction: Direction, r#type: NumberConstants }, // If particle at direction is of type X
    CheckTypesInDirection { direction: Direction, types: Vec<NumberConstants> }, // If particle at direction is of type X
    Not { block: Condition }, // Negates a block result, it's inverting a boolean
    And { block1: Condition, block2: Condition }, // Logical AND
    Or { block1: Condition, block2: Condition }, // Logical OR
    IsTouching { r#type: NumberConstants }, // Looks neighbour to see if it's of type X
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
                let particle_id = r#type.to_number(api) as u8;

                Box::new(move |plugin, particle, api| {
                    let direction = api.get_transformation().transform(&direction);
                    api.get(direction[0], direction[1]).id == particle_id
                })
            }
            Conditions::CheckTypesInDirection { direction, types } => {
                let types = types
                    .iter()
                    .map(|particle| particle.to_number(api) as u8)
                    .collect::<Vec<_>>();

                // If the array is only one element, we can optimize it by taking it out.
                if types.len() == 1{
                    let particle_id = types[0];
                    Box::new(move |plugin, particle, api| {
                        let direction = api.get_transformation().transform(&direction);
                        api.get_type(direction[0], direction[1]) == particle_id
                    })
                }
                else {   
                    Box::new(move |plugin, particle, api| {
                        let direction = api.get_transformation().transform(&direction);
                        api.is_any_particle_at(direction[0], direction[1], &types)
                    })
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
                let r#type = r#type.to_number(api) as u8;

                Box::new(move |plugin, particle, api| {
                    ParticleApi::NEIGHBORS
                        .iter()
                        .any(|(direction)| api.get_type(direction.x, direction.y) == r#type)
                })
            },
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
                    (Number::NumbersRuntime(runtime1), Number::NumberConstants(constant2)) => {
                        let number2 = constant2.to_number(api);
                        Box::new(move |plugin, particle, api| runtime1.to_number(api) == number2)
                    }
                    (Number::NumberConstants(constant1), Number::NumbersRuntime(runtime2)) => {
                        let number1 = constant1.to_number(api);
                        Box::new(move |plugin, particle, api| number1 == runtime2.to_number(api))
                    }
                    (Number::NumberConstants(constant1), Number::NumberConstants(constant2)) => {
                        let number1 = constant1.to_number(api);
                        let number2 = constant2.to_number(api);
                        let result = number1 == number2;
                        Box::new(move |plugin, particle, api| result)
                    }
                }
            },
            Conditions::CompareBiggerThan { block1, block2 } => match (block1, block2) {
                (Number::NumbersRuntime(runtime1), Number::NumbersRuntime(runtime2)) => {
                    Box::new(move |plugin, particle, api| {
                        runtime1.to_number(api) > runtime2.to_number(api)
                    })
                }
                (Number::NumbersRuntime(runtime1), Number::NumberConstants(constant2)) => {
                    let number2 = constant2.to_number(api);
                    Box::new(move |plugin, particle, api| runtime1.to_number(api) > number2)
                }
                (Number::NumberConstants(constant1), Number::NumbersRuntime(runtime2)) => {
                    let number1 = constant1.to_number(api);
                    Box::new(move |plugin, particle, api| number1 > runtime2.to_number(api))
                }
                (Number::NumberConstants(constant1), Number::NumberConstants(constant2)) => {
                    let number1 = constant1.to_number(api);
                    let number2 = constant2.to_number(api);
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
                (Number::NumbersRuntime(runtime1), Number::NumberConstants(constant2)) => {
                    let number2 = constant2.to_number(api);
                    Box::new(move |plugin, particle, api| runtime1.to_number(api) < number2)
                }
                (Number::NumberConstants(constant1), Number::NumbersRuntime(runtime2)) => {
                    let number1 = constant1.to_number(api);
                    Box::new(move |plugin, particle, api| number1 < runtime2.to_number(api))
                }
                (Number::NumberConstants(constant1), Number::NumberConstants(constant2)) => {
                    let number1 = constant1.to_number(api);
                    let number2 = constant2.to_number(api);
                    let result = number1 < number2;
                    Box::new(move |plugin, particle, api| result)
                }
            },
            Conditions::Boolean { value } => Box::new(move |plugin, particle, api| value),
            Conditions::IsEmpty { direction } => {
                Box::new(move |plugin, particle, api| api.is_empty(direction[0], direction[1]))
            },
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
