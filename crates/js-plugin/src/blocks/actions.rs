use super::*;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "action", content = "data", rename_all = "camelCase")]
#[rustfmt::skip]
pub enum Actions
{
    Swap { direction: Direction },
    CopyTo { direction: Direction },
    ChangeInto { direction: Direction, r#type: Number },
    RandomTransformation { transformation: TransformationInternal, block: Option<Vec<Actions>>},
    ForEachTransformation { transformation: TransformationInternal, block: Option<Vec<Actions>>},
    RotatedBy { number: Number, block: Option<Vec<Actions>> },
    If (Vec<Option<(Conditions, Vec<Actions>)>>), 
    IncreaseParticlePropierty { propierty: ParticlePropierties, number: Number,  direction: Direction },
    SetParticlePropierty { propierty: ParticlePropierties, number: Number, direction: Direction },
    Repeat { number: Number, block: Option<Vec<Actions>> },
    EveryXFrames { number: Number, block: Option<Vec<Actions>> },
    None
}

impl Actions {
    pub fn to_func(
        &self,
        api: &ParticleApi,
    ) -> Box<dyn Fn(&JSPlugin, &mut ParticleApi) -> ()> {
        let block = self.clone();
        match block {
            Actions::Swap { direction } => match direction {
                Direction::Constant(direction) => {
                    let direction = direction;
                    Box::new(move |_, api| {
                        let direction = api.get_transformation().transform(&direction);
                        api.swap(direction[0], direction[1]);
                    })
                }
                _ => Box::new(move |_, api| {
                    let direction = direction.get_direction(api);
                    let direction = api.get_transformation().transform(&direction);
                    api.swap(direction[0], direction[1]);
                }),
            },
            Actions::CopyTo { direction } => match direction {
                Direction::Constant(direction) => {
                    let direction = direction;
                    Box::new(move |_, api| {
                        let direction = api.get_transformation().transform(&direction);
                        api.set(direction[0], direction[1], api.get_current());
                    })
                }
                _ => Box::new(move |_, api| {
                    let direction = direction.get_direction(api);
                    let direction = api.get_transformation().transform(&direction);
                    api.set(direction[0], direction[1], api.get_current());
                }),
            },
            // TODO
            Actions::ChangeInto { direction, r#type } => {
                match r#type {
                    Number::FromID(r#type) => {
                        let r#type = r#type as u8;
                        if r#type >= api.get_particle_count() {
                            return Box::new(|_, _| ());
                        }
                        let particle_id = r#type;
                        match direction {
                            Direction::Constant(direction) => {
                                let direction = direction;
                                Box::new(move |_, api| {
                                    let direction = api.get_transformation().transform(&direction);
                                    api.set(direction[0], direction[1], api.new_particle(particle_id));
                                })
                            }
                            _ => Box::new(move |_, api| {
                                let direction = direction.get_direction(api);
                                let direction = api.get_transformation().transform(&direction);
                                api.set(direction[0], direction[1], api.new_particle(particle_id));
                            }),
                        }
                    }
                    _ => 
                    {
                        match direction {
                            Direction::Constant(direction) => {
                                let direction = direction;
                                Box::new(move |_, api| {
                                    let direction = api.get_transformation().transform(&direction);
                                    let particle_id = r#type.to_number(api) as u8;

                                    if particle_id >= api.get_particle_count() {
                                        return;
                                    }

                                    api.set(direction[0], direction[1], api.new_particle(particle_id));
                                })
                            }
                            _ => Box::new(move |_, api| {
                                let direction = direction.get_direction(api);
                                let direction = api.get_transformation().transform(&direction);

                                let particle_id = r#type.to_number(api) as u8;

                                    if particle_id >= api.get_particle_count() {
                                        return;
                                    }
                                api.set(direction[0], direction[1], api.new_particle(particle_id));
                            }),
                        }
                    }
                }
            }

            Actions::RandomTransformation {
                transformation,
                block,
            } => {
                if block.is_none() {
                    return Box::new(|_, _| ());
                }

                let block = block.unwrap();
                let func = block
                    .iter()
                    .map(|block| block.to_func(api))
                    .collect::<Vec<_>>();

                Box::new(move |plugin, api| {
                    let previous_trasnformation = api.get_transformation().clone();

                    let transformation = transformation.to_transformation(api);
                    api.set_transformation(transformation);

                    func.iter().for_each(|func| func(plugin, api));

                    api.set_transformation(previous_trasnformation);
                })
            }
            // This is a for.. This shoould not return a bool, this should be separated into other enum
            Actions::ForEachTransformation {
                transformation,
                block,
            } => {
                if block.is_none() {
                    return Box::new(|_, _| ());
                }

                let block = block.unwrap();

                let func = block
                    .iter()
                    .map(|block| block.to_func(api))
                    .collect::<Vec<_>>();

                match transformation {
                    TransformationInternal::HorizontalReflection => {
                        Box::new(move |plugin, api| {
                            let previous_trasnformation = api.get_transformation().clone();

                            let transformation = Transformation::HorizontalReflection(true);
                            api.set_transformation(transformation);

                            func.iter().for_each(|func| func(plugin, api));

                            let transformation = Transformation::HorizontalReflection(false);
                            api.set_transformation(transformation);

                            func.iter().for_each(|func| func(plugin, api));

                            api.set_transformation(previous_trasnformation);
                        })
                    }
                    TransformationInternal::VerticalReflection => {
                        Box::new(move |plugin, api| {
                            let previous_trasnformation = api.get_transformation().clone();

                            let transformation = Transformation::VerticalReflection(true);
                            api.set_transformation(transformation);

                            func.iter().for_each(|func| func(plugin, api));

                            let transformation = Transformation::VerticalReflection(false);
                            api.set_transformation(transformation);

                            func.iter().for_each(|func| func(plugin, api));

                            api.set_transformation(previous_trasnformation);
                        })
                    }
                    TransformationInternal::Reflection => Box::new(move |plugin, api| {
                        let previous_trasnformation = api.get_transformation().clone();

                        let transformation = Transformation::Reflection(true, true);
                        api.set_transformation(transformation);

                        func.iter().for_each(|func| func(plugin, api));

                        let transformation = Transformation::Reflection(false, false);
                        api.set_transformation(transformation);

                        func.iter().for_each(|func| func(plugin, api));

                        let transformation = Transformation::Reflection(false, true);
                        api.set_transformation(transformation);

                        func.iter().for_each(|func| func(plugin, api));

                        let transformation = Transformation::Reflection(true, false);
                        api.set_transformation(transformation);

                        func.iter().for_each(|func| func(plugin, api));

                        api.set_transformation(previous_trasnformation);
                    }),
                    TransformationInternal::Rotation => Box::new(move |plugin, api| {
                        let previous_transformation = api.get_transformation().clone();

                        for i in 0..=7 {
                            let transformation = Transformation::Rotation(i);
                            api.set_transformation(transformation);

                            func.iter().for_each(|func| func(plugin, api));
                        }

                        api.set_transformation(previous_transformation);
                    }),
                    TransformationInternal::None => Box::new(move |plugin, api| {
                        func.iter().for_each(|func| func(plugin, api))
                    }),
                }
            }
            Actions::If(blocks) => {
                let non_none_blocks = blocks
                    .iter()
                    .filter(|block| block.is_some())
                    .collect::<Vec<_>>();

                if non_none_blocks.is_empty() {
                    return Box::new(|_, _| ());
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

                Box::new(move |plugin, api| {
                    // We will iterate until we find a condition that is true, exeduting the block and return
                    non_none_blocks
                        .iter()
                        .find(|(condition, _)| condition(plugin, api))
                        .map(|(_, action)| {
                            action.iter().for_each(|func| func(plugin, api))
                        });
                })
            }
            Actions::RotatedBy { number, block } => {
                if block.is_none() {
                    return Box::new(|_, _| ());
                }

                let block = block.unwrap();
                let func = block
                    .iter()
                    .map(|block| block.to_func(api))
                    .collect::<Vec<_>>();
                Box::new(move |plugin, api| {
                    let previous_transformation = api.get_transformation().clone();
                    let rotations = number.to_number(api);
                    // As this is a runtime number, we have to force it to be between 0 and 7 using modulo
                    let rotations = rotations.rem_euclid(8);
                    let transformation = Transformation::Rotation(rotations as usize);
                    api.set_transformation(transformation);
                    func.iter().for_each(|func| func(plugin, api));
                    api.set_transformation(previous_transformation);
                })
            }
            Actions::IncreaseParticlePropierty {
                propierty,
                number,
                direction,
            } => {
                match propierty {
                    ParticlePropierties::Opacity => Box::new(move |_, api| {
                        let direction = direction.get_direction(api);
                        let direction = api.get_transformation().transform(&direction);
                        let number = number.to_number(api) as i8;
                        let mut particle = api.get(direction[0], direction[1]);
                        particle.opacity = particle.opacity.saturating_add_signed(number).min(100); // This is to avoid overflow
                        api.set_relaxed(direction[0], direction[1], particle);
                    }),
                    ParticlePropierties::Extra => Box::new(move |_, api| {
                        let direction = direction.get_direction(api);
                        let direction = api.get_transformation().transform(&direction);
                        let number = number.to_number(api) as i8;
                        let mut particle = api.get(direction[0], direction[1]);
                        particle.extra = particle.extra.saturating_add_signed(number).min(100); // This is to avoid overflow
                        api.set_relaxed(direction[0], direction[1], particle);
                    }),
                    ParticlePropierties::HueShift => Box::new(move |_, api| {
                        let direction = direction.get_direction(api);
                        let direction = api.get_transformation().transform(&direction);
                        let number = number.to_number(api) as i8;
                        let mut particle = api.get(direction[0], direction[1]);
                        particle.hue_shift = (particle.hue_shift.saturating_add_signed(number) % 101).max(0); // This is to avoid overflow
                        api.set_relaxed(direction[0], direction[1], particle);
                    }),
                    ParticlePropierties::Extra2 => Box::new(move |_, api| {
                        let direction = direction.get_direction(api);
                        let direction = api.get_transformation().transform(&direction);
                        let number = number.to_number(api) as i8;
                        let mut particle = api.get(direction[0], direction[1]);
                        particle.extra2 = particle.extra2.saturating_add_signed(number).min(100); // This is to avoid overflow
                        api.set_relaxed(direction[0], direction[1], particle);
                    }),
                    ParticlePropierties::Extra3 => Box::new(move |_, api| {
                        let direction = direction.get_direction(api);
                        let direction = api.get_transformation().transform(&direction);
                        let number = number.to_number(api) as i8;
                        let mut particle = api.get(direction[0], direction[1]);
                        particle.extra3 = particle.extra3.saturating_add_signed(number).min(100); // This is to avoid overflow
                        api.set_relaxed(direction[0], direction[1], particle);
                    }),
                    ParticlePropierties::ColorFade => Box::new(move |_, api| {
                        let direction = direction.get_direction(api);
                        let direction = api.get_transformation().transform(&direction);
                        let number = number.to_number(api) as i8;
                        let mut particle = api.get(direction[0], direction[1]);
                        particle.color_fade = particle.color_fade.saturating_add_signed(number).min(100); // This is to avoid overflow
                        api.set_relaxed(direction[0], direction[1], particle);
                    }),
                }
            }
            Actions::SetParticlePropierty {
                propierty,
                number,
                direction,
            } => match propierty {
                ParticlePropierties::Opacity => Box::new(move |_, api| {
                    let direction = direction.get_direction(api);
                    let direction = api.get_transformation().transform(&direction);
                    let number = number.to_number(api).clamp(0, 100) as u8;
                    let mut particle = api.get(direction[0], direction[1]);
                    particle.opacity = number;
                    api.set_relaxed(direction[0], direction[1], particle);
                }),
                ParticlePropierties::Extra => Box::new(move |_, api| {
                    let direction = direction.get_direction(api);
                    let direction = api.get_transformation().transform(&direction);
                    let number = number.to_number(api).clamp(0, 100) as u8;
                    let mut particle = api.get(direction[0], direction[1]);                        
                    particle.extra = number;
                    api.set_relaxed(direction[0], direction[1], particle);
                }),
                ParticlePropierties::HueShift => Box::new(move |_, api| {
                    let direction = direction.get_direction(api);
                    let direction = api.get_transformation().transform(&direction);
                    let number = number.to_number(api).clamp(0, 100) as u8;
                    let mut particle = api.get(direction[0], direction[1]);                        
                    particle.hue_shift = number;
                    api.set_relaxed(direction[0], direction[1], particle);
                }),
                ParticlePropierties::Extra2 => Box::new(move |_, api| {
                    let direction = direction.get_direction(api);
                    let direction = api.get_transformation().transform(&direction);
                    let number = number.to_number(api).clamp(0, 100) as u8;
                    let mut particle = api.get(direction[0], direction[1]);                        
                    particle.extra2 = number;
                    api.set_relaxed(direction[0], direction[1], particle);
                }),
                ParticlePropierties::Extra3 => Box::new(move |_, api| {
                    let direction = direction.get_direction(api);
                    let direction = api.get_transformation().transform(&direction);
                    let number = number.to_number(api).clamp(0, 100) as u8;
                    let mut particle = api.get(direction[0], direction[1]);                        
                    particle.extra3 = number;
                    api.set_relaxed(direction[0], direction[1], particle);
                }),
                ParticlePropierties::ColorFade => Box::new(move |_, api| {
                    let direction = direction.get_direction(api);
                    let direction = api.get_transformation().transform(&direction);
                    let number = number.to_number(api).clamp(0, 100) as u8;
                    let mut particle = api.get(direction[0], direction[1]);                        
                    particle.color_fade = number;
                    api.set_relaxed(direction[0], direction[1], particle);
                }),
            },
            Actions::Repeat { number, block } => {
                if block.is_none() {
                    return Box::new(|_, _| ());
                }

                let block = block.unwrap();
                let func = block
                    .iter()
                    .map(|block| block.to_func(api))
                    .collect::<Vec<_>>();

                Box::new(move |plugin, api| {
                    let times = number.to_number(api);
                    for _ in 0..times {
                        func.iter().for_each(|func| func(plugin, api));
                    }
                })
            }
            Actions::EveryXFrames { number, block } => {
                if block.is_none() {
                    return Box::new(|_, _| ());
                }

                let block = block.unwrap();
                let func = block
                    .iter()
                    .map(|block| block.to_func(api))
                    .collect::<Vec<_>>();

                Box::new(move |plugin, api| {
                    let frames = number.to_number(api) as u32;

                    // We don't want to divide by 0 xD
                    // And negative numbers are not allowed, they don't make sense
                    if frames <= 0 {
                        return;
                    }
                    
                    // Print frames and api frame count and whether they are equal
                    if api.get_frame_count() % frames == 0 {
                        func.iter().for_each(|func| func(plugin, api));
                    }
                    
                    api.set(0, 0, api.get_current());
                })
            }
            Actions::None => Box::new(|_, _| ()),
        }
    }
}
