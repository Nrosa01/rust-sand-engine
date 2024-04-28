use super::*;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "action", content = "data", rename_all = "camelCase")]
#[rustfmt::skip]
pub enum Actions
{
    Swap { direction: Direction },
    CopyTo { direction: Direction },
    ChangeInto { direction: Direction, r#type: ParticleType },
    RandomTransformation { transformation: TransformationInternal, block: Action},
    ForEachTransformation { transformation: TransformationInternal, block: Action},
    RotatedBy { number: Number, block: Action },
    If { condition: Conditions, result: Action, r#else: Option<Action>}, // If block, if true, execute action
    IncreaseParticlePropierty { propierty: ParticlePropierties, number: Number,  direction: Option<Direction> },
    SetParticlePropierty { propierty: ParticlePropierties, number: Number, direction: Option<Direction> },
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

                // As the particle is invalid, all this block is invalid
                if particle_id == Particle::INVALID.id {
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
                let func = block.to_func(api);
                Box::new(move |plugin, particle, api| {
                    let previous_transformation = api.get_transformation().clone();
                    let rotations = number.to_number(api);
                    // As this is a runtime number, we have to force it to be between 0 and 7 using modulo
                    let rotations = rotations % 8;
                    let transformation = Transformation::Rotation(rotations as usize);
                    api.set_transformation(transformation);
                    func(plugin, particle, api);
                    api.set_transformation(previous_transformation);
                })
            }
            Actions::IncreaseParticlePropierty {
                propierty,
                number,
                direction,
            } => {
                if direction.is_some() {
                    let direction = direction.unwrap();
                    match propierty {
                        ParticlePropierties::Light => Box::new(move |_, _, api| {
                            let direction = direction.get_direction(api);
                            let direction = api.get_transformation().transform(&direction);
                            let number = number.to_number(api) as i8;
                            let mut particle = api.get(direction[0], direction[1]);
                            particle.light = particle.light.saturating_add_signed(number).min(100); // This is to avoid overflow
                            api.set(0, 0, particle);
                        }),
                        ParticlePropierties::Extra => Box::new(move |_, _, api| {
                            let direction = direction.get_direction(api);
                            let direction = api.get_transformation().transform(&direction);
                            let number = number.to_number(api) as i8;
                            let mut particle = api.get(direction[0], direction[1]);
                            particle.extra = particle.extra.saturating_add_signed(number).min(100); // This is to avoid overflow
                            api.set(0, 0, particle);
                        }),
                    } 
                } else {
                    match propierty {
                        ParticlePropierties::Light => Box::new(move |_, particle, api| {
                            let number = number.to_number(api) as i8;
                            let mut particle = particle;
                            particle.light = particle.light.saturating_add_signed(number).min(100); // This is to avoid overflow
                            api.set(0, 0, particle);
                        }),
                        ParticlePropierties::Extra => Box::new(move |_, particle, api| {
                            let number = number.to_number(api) as i8;
                            let mut particle = particle;
                            particle.extra = particle.extra.saturating_add_signed(number).min(100); // This is to avoid overflow
                            api.set(0, 0, particle);
                        }),
                    }
                }
            }
            Actions::SetParticlePropierty {
                propierty,
                number,
                direction,
            } => 
            {
                if direction.is_some()
                {
                    let direction = direction.unwrap();
                    match propierty {
                        ParticlePropierties::Light => Box::new(move |_, _, api| {
                            let direction = direction.get_direction(api);
                            let direction = api.get_transformation().transform(&direction);
                            let number = number.to_number(api) as u8;
                            let mut particle = api.get(direction[0], direction[1]);
                            particle.light = number.min(100);
                            api.set(0, 0, particle);
                        }),
                        ParticlePropierties::Extra => Box::new(move |_, _, api| {
                            let direction = direction.get_direction(api);
                            let direction = api.get_transformation().transform(&direction);
                            let number = number.to_number(api) as u8;
                            let mut particle = api.get(direction[0], direction[1]);
                            particle.extra = number.min(100);
                            api.set(0, 0, particle);
                        }),
                    }
                    
                }
                else {
                    match propierty {
                        ParticlePropierties::Light => Box::new(move |_, particle, api| {
                            let number = number.to_number(api) as u8;
                            let mut particle = particle;
                            particle.light = number.min(100);
                            api.set(0, 0, particle);
                        }),
                        ParticlePropierties::Extra => Box::new(move |_, particle, api| {
                            let number = number.to_number(api) as u8;
                            let mut particle = particle;
                            particle.extra = number.min(100);
                            api.set(0, 0, particle);
                        }),
                    }
                }
            }
            Actions::Repeat { number, block } => {
                let func = block.to_func(api);

                Box::new(move |plugin, particle, api| {
                    let times = number.to_number(api);
                    for _ in 0..times {
                        func(plugin, particle, api);
                    }
                })
            }
            Actions::EveryXFrames { number, block } => {
                let func = block.to_func(api);

                Box::new(move |plugin, particle, api| {
                    let frames = number.to_number(api) as u32;
                    if api.get_frame() % frames == 0 {
                        func(plugin, particle, api);
                    }
                })
            }
        }
    }
}
