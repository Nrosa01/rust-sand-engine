use super::*;

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
