use super::*;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "action", content = "data", rename_all = "camelCase")]
#[rustfmt::skip]
pub enum Actions
{
    Swap { direction: Direction },
    CopyTo { direction: Direction },
    ChangeInto { direction: Direction, r#type: ParticleType },
    RandomTransformation { transformation: TransformationInternal, block: Option<Vec<Action>>},
    ForEachTransformation { transformation: TransformationInternal, block: Option<Vec<Action>>},
    RotatedBy { number: Number, block: Option<Vec<Action>> },
    If (Vec<Option<(Conditions, Vec<Actions>)>>), 
    IncreaseParticlePropierty { propierty: ParticlePropierties, number: Number,  direction: Direction },
    SetParticlePropierty { propierty: ParticlePropierties, number: Number, direction: Direction },
    Repeat { number: Number, block: Option<Vec<Action>> },
    EveryXFrames { number: Number, block: Option<Vec<Action>> },
    None
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

                // As the particle is invalid, all this block is invalid
                if particle_id >= api.get_particle_count() {
                    return Box::new(|_, _, _| ());
                }

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
                if block.is_none() {
                    return Box::new(|_, _, _| ());
                }

                let block = block.unwrap();
                let func = block
                    .iter()
                    .map(|block| block.to_func(api))
                    .collect::<Vec<_>>();

                Box::new(move |plugin, particle, api| {
                    let previous_trasnformation = api.get_transformation().clone();

                    let transformation = transformation.to_transformation(api);
                    api.set_transformation(transformation);

                    func.iter().for_each(|func| func(plugin, particle, api));

                    api.set_transformation(previous_trasnformation);
                })
            }
            // This is a for.. This shoould not return a bool, this should be separated into other enum
            Actions::ForEachTransformation {
                transformation,
                block,
            } => {
                if block.is_none() {
                    return Box::new(|_, _, _| ());
                }

                let block = block.unwrap();

                let func = block
                    .iter()
                    .map(|block| block.to_func(api))
                    .collect::<Vec<_>>();

                match transformation {
                    TransformationInternal::HorizontalReflection => {
                        Box::new(move |plugin, particle, api| {
                            let previous_trasnformation = api.get_transformation().clone();

                            let transformation = Transformation::HorizontalReflection(true);
                            api.set_transformation(transformation);

                            func.iter().for_each(|func| func(plugin, particle, api));

                            let transformation = Transformation::HorizontalReflection(false);
                            api.set_transformation(transformation);

                            func.iter().for_each(|func| func(plugin, particle, api));

                            api.set_transformation(previous_trasnformation);
                        })
                    }
                    TransformationInternal::VerticalReflection => {
                        Box::new(move |plugin, particle, api| {
                            let previous_trasnformation = api.get_transformation().clone();

                            let transformation = Transformation::VerticalReflection(true);
                            api.set_transformation(transformation);

                            func.iter().for_each(|func| func(plugin, particle, api));

                            let transformation = Transformation::VerticalReflection(false);
                            api.set_transformation(transformation);

                            func.iter().for_each(|func| func(plugin, particle, api));

                            api.set_transformation(previous_trasnformation);
                        })
                    }
                    TransformationInternal::Reflection => Box::new(move |plugin, particle, api| {
                        let previous_trasnformation = api.get_transformation().clone();

                        let transformation = Transformation::Reflection(true, true);
                        api.set_transformation(transformation);

                        func.iter().for_each(|func| func(plugin, particle, api));

                        let transformation = Transformation::Reflection(false, false);
                        api.set_transformation(transformation);

                        func.iter().for_each(|func| func(plugin, particle, api));

                        let transformation = Transformation::Reflection(false, true);
                        api.set_transformation(transformation);

                        func.iter().for_each(|func| func(plugin, particle, api));

                        let transformation = Transformation::Reflection(true, false);
                        api.set_transformation(transformation);

                        func.iter().for_each(|func| func(plugin, particle, api));

                        api.set_transformation(previous_trasnformation);
                    }),
                    TransformationInternal::Rotation => Box::new(move |plugin, particle, api| {
                        let previous_transformation = api.get_transformation().clone();

                        for i in 1..=7 {
                            let transformation = Transformation::Rotation(i);
                            api.set_transformation(transformation);

                            func.iter().for_each(|func| func(plugin, particle, api));
                        }

                        api.set_transformation(previous_transformation);
                    }),
                    TransformationInternal::None => Box::new(move |plugin, particle, api| {
                        func.iter().for_each(|func| func(plugin, particle, api))
                    }),
                }
            }
            Actions::If(blocks) => {
                let non_none_blocks = blocks
                    .iter()
                    .filter(|block| block.is_some())
                    .collect::<Vec<_>>();

                if non_none_blocks.is_empty() {
                    return Box::new(|_, _, _| ());
                }

                // Convert to an array of tuples that are functions
                let non_none_blocks = non_none_blocks
                    .iter()
                    .map(|block| {
                        let (condition, action) = block.as_ref().unwrap();
                        let condition = condition.to_func(api);
                        let action = action
                            .iter()
                            .map(|block| block.to_func(api))
                            .collect::<Vec<_>>();
                        (condition, action)
                    })
                    .collect::<Vec<_>>();

                Box::new(move |plugin, particle, api| {
                    // We will iterate until we find a condition that is true, exeduting the block and return
                    non_none_blocks.iter()
                        .find(|(condition, _)| condition(plugin, particle, api))
                        .map(|(_, action)| action.iter().for_each(|func| func(plugin, particle, api)));
                })
            }
            Actions::RotatedBy { number, block } => {
                if block.is_none() {
                    return Box::new(|_, _, _| ());
                }

                let block = block.unwrap();
                let func = block
                    .iter()
                    .map(|block| block.to_func(api))
                    .collect::<Vec<_>>();
                Box::new(move |plugin, particle, api| {
                    let previous_transformation = api.get_transformation().clone();
                    let rotations = number.to_number(api);
                    // As this is a runtime number, we have to force it to be between 0 and 7 using modulo
                    let rotations = rotations.rem_euclid(8);
                    let transformation = Transformation::Rotation(rotations as usize);
                    api.set_transformation(transformation);
                    func.iter().for_each(|func| func(plugin, particle, api));
                    api.set_transformation(previous_transformation);
                })
            }
            Actions::IncreaseParticlePropierty {
                propierty,
                number,
                direction,
            } => {
                match propierty {
                    ParticlePropierties::Light => Box::new(move |_, _, api| {
                        let direction = direction.get_direction(api);
                        let direction = api.get_transformation().transform(&direction);
                        let number = number.to_number(api) as i8;
                        let mut particle = api.get(direction[0], direction[1]);
                        particle.light = particle.light.saturating_add_signed(number).min(100); // This is to avoid overflow
                        api.set(direction[0], direction[1], particle);
                    }),
                    ParticlePropierties::Extra => Box::new(move |_, _, api| {
                        let direction = direction.get_direction(api);
                        let direction = api.get_transformation().transform(&direction);
                        let number = number.to_number(api) as i8;
                        let mut particle = api.get(direction[0], direction[1]);
                        particle.extra = particle.extra.saturating_add_signed(number).min(100); // This is to avoid overflow
                        api.set(direction[0], direction[1], particle);
                    }),
                }
            }
            Actions::SetParticlePropierty {
                propierty,
                number,
                direction,
            } => match propierty {
                ParticlePropierties::Light => Box::new(move |_, _, api| {
                    let direction = direction.get_direction(api);
                    let direction = api.get_transformation().transform(&direction);
                    let number = number.to_number(api).clamp(0, 100) as u8;
                    let mut particle = api.get(direction[0], direction[1]);
                    particle.light = number;
                    api.set(direction[0], direction[1], particle);
                }),
                ParticlePropierties::Extra => Box::new(move |_, _, api| {
                    let direction = direction.get_direction(api);
                    let direction = api.get_transformation().transform(&direction);
                    let number = number.to_number(api).clamp(0, 100) as u8;
                    let mut particle = api.get(direction[0], direction[1]);
                    particle.extra = number;
                    api.set(direction[0], direction[1], particle);
                }),
            },
            Actions::Repeat { number, block } => {
                if block.is_none() {
                    return Box::new(|_, _, _| ());
                }

                let block = block.unwrap();
                let func = block
                    .iter()
                    .map(|block| block.to_func(api))
                    .collect::<Vec<_>>();

                Box::new(move |plugin, particle, api| {
                    let times = number.to_number(api);
                    for _ in 0..times {
                        func.iter().for_each(|func| func(plugin, particle, api));
                    }
                })
            }
            Actions::EveryXFrames { number, block } => {
                if block.is_none() {
                    return Box::new(|_, _, _| ());
                }

                let block = block.unwrap();
                let func = block
                    .iter()
                    .map(|block| block.to_func(api))
                    .collect::<Vec<_>>();

                Box::new(move |plugin, particle, api| {
                    let frames = number.to_number(api) as u32;

                    if frames == 0 {
                        return; // We don't want to divide by 0 xD
                    }

                    if api.get_frame() % frames == 0 {
                        func.iter().for_each(|func| func(plugin, particle, api));
                    }
                })
            }
            Actions::None => Box::new(|_, _, _| ()),
        }
    }
}
