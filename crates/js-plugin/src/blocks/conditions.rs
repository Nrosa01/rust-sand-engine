use self::numbers::ParticleType;

use super::*;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "block", content = "data", rename_all = "camelCase")]
#[rustfmt::skip]
pub enum Conditions {
    CheckTypesInDirection { direction: Direction, types: Vec<ParticleType> }, // If particle at direction is of type X
    Not { block: Condition }, // Negates a block result, it's inverting a boolean
    And { block1: Condition, block2: Condition }, // Logical AND
    Or { block1: Condition, block2: Condition }, // Logical OR
    IsTouching { r#type: Vec<ParticleType> }, // Looks neighbour to see if it's of type X
    OneInXChance { chance: Number }, // Returns true one in a X chance, for example, if X is 3, it will return true 1/3 of the time
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
    ) -> Box<dyn Fn(&JSPlugin, &mut ParticleApi) -> bool> {
        let block = self.clone();
        match block {
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
                            Box::new(move |plugin, api| {
                                let direction = api.get_transformation().transform(&direction);
                                api.get_type(direction[0], direction[1]) == particle_id
                            })
                        }
                        _ => Box::new(move |plugin, api| {
                            let direction = direction.get_direction(api);
                            let direction = api.get_transformation().transform(&direction);
                            api.get_type(direction[0], direction[1]) == particle_id
                        }),
                    }
                } else {
                    match direction {
                        Direction::Constant(direction) => {
                            let direction = direction;
                            Box::new(move |plugin, api| {
                                let direction = api.get_transformation().transform(&direction);
                                api.is_any_particle_at(direction[0], direction[1], &types)
                            })
                        }
                        _ => Box::new(move |plugin, api| {
                            let direction = direction.get_direction(api);
                            let direction = api.get_transformation().transform(&direction);
                            api.is_any_particle_at(direction[0], direction[1], &types)
                        }),
                    }
                }
            }
            Conditions::Not { block } => {
                let func = block.to_func(api);

                Box::new(move |plugin, api| !func(plugin, api))
            }
            Conditions::And { block1, block2 } => {
                let func1 = block1.to_func(api);
                let func2 = block2.to_func(api);

                Box::new(move |plugin, api| {
                    func1(plugin, api) && func2(plugin, api)
                })
            }
            Conditions::Or { block1, block2 } => {
                let func1 = block1.to_func(api);
                let func2 = block2.to_func(api);

                Box::new(move |plugin, api| {
                    func1(plugin, api) || func2(plugin, api)
                })
            }
            Conditions::IsTouching { r#type } => {
                let types = r#type
                    .iter()
                    .map(|particle| particle.get_particle_id(api) as u8)
                    .collect::<Vec<_>>();

                if types.len() == 1 {
                    let r#type = types[0];

                    Box::new(move |plugin, api| {
                        ParticleApi::NEIGHBORS
                            .iter()
                            .any(|(direction)| api.get_type(direction.x, direction.y) == r#type)
                    })
                } else {
                    Box::new(move |plugin, api| {
                        ParticleApi::NEIGHBORS.iter().any(|(direction)| {
                            types.contains(&api.get_type(direction.x, direction.y))
                        })
                    })
                }
            }
            Conditions::CompareNumberEquality { block1, block2 } => {
                Box::new(move |plugin, api| {
                    let number1 = block1.to_number(api);
                    let number2 = block2.to_number(api);
                    number1 == number2
                })
            }
            Conditions::CompareBiggerThan { block1, block2 } => {
                Box::new(move |plugin, api| {
                    let number1 = block1.to_number(api);
                    let number2 = block2.to_number(api);
                    number1 > number2
                })
            }
            Conditions::CompareLessThan { block1, block2 } => {
                Box::new(move |plugin, api| {
                    let number1 = block1.to_number(api);
                    let number2 = block2.to_number(api);
                    number1 < number2
                })
            }
            Conditions::Boolean { value } => Box::new(move |plugin, api| value),
            Conditions::IsEmpty { direction } => match direction {
                Direction::Constant(direction) => {
                    let direction = direction;
                    Box::new(move |_, api| {
                        let direction = api.get_transformation().transform(&direction);
                        api.is_empty(direction[0], direction[1])
                    })
                }
                _ => Box::new(move |_, api| {
                    let direction = direction.get_direction(api);
                    let direction = api.get_transformation().transform(&direction);
                    api.is_empty(direction[0], direction[1])
                }),
            },
            Conditions::OneInXChance { chance } => match chance {
                Number::Constant(number) => {
                    let number = number as i32;
                    if number <= 1 {
                        return Box::new(move |_, _| true);
                    }
                    else {
                        Box::new(move |_, api| {
                            let random_number = api.gen_range(1, number);
                            random_number == 1
                        })
                    }
                }
                number => Box::new(move |plugin, api| {
                    let chance = number.to_number(api);
                    let random_number = api.gen_range(1, chance);
                    random_number == 1
                }),
            },
            Conditions::CompareBooleans { block1, block2 } => {
                let func1 = block1.to_func(api);
                let func2 = block2.to_func(api);

                Box::new(move |plugin, api| {
                    func1(plugin, api) == func2(plugin, api)
                })
            }
        }
    }
}
