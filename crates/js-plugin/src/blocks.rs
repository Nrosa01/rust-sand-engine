use app_core::{Particle, ParticleApi};
use serde::{Deserialize, Serialize};

use crate::plugins::JSPlugin;

// type Action = Box<dyn Fn(Particle, &mut ParticleApi)>;
type Action = Box<Blocks>;
type Condition = Box<Blocks>;
type Direction = [i32; 2];
type ParticleType = u8;
type NumberLiteral = usize;

// Right now we assume users now the ID of the particles they want to check against
// Once we get this working, we are going to transform de json and substitute
// The particles names with an index that will be indexed into an array that has the particle IDs.

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "number", rename_all = "camelCase")]
pub enum Numbers {
    ParticleType(ParticleType),
    Number(NumberLiteral),
    NumberOfXTouching(ParticleType),
    TypeOf(Direction),
}

impl Numbers {
    pub fn to_number(&self, _: &JSPlugin, _: Particle, api: &mut ParticleApi) -> usize {
        match self {
            Numbers::ParticleType(particle_type) => *particle_type as usize,
            Numbers::Number(number) => *number,
            Numbers::NumberOfXTouching(particle_type) => ParticleApi::NEIGHBORS
                .iter()
                .filter(|dir| api.get_type(dir.x, dir.y) == *particle_type)
                .count(),
            Numbers::TypeOf(direction) => api.get_type(direction[0], direction[1]) as usize,
        }
    }
}

// Taken from Sandspiel Studio
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "block", content = "data", rename_all = "camelCase")]
#[rustfmt::skip]
pub enum Blocks {
    Swap { direction: Direction },
    CopyTo { direction: Direction },
    ChangeInto { direction: Direction, r#type: ParticleType },
    IfDirectionIsType { direction: Direction, r#type: ParticleType }, // If particle at direction is of type X
    Not { block: Condition }, // Negates a block result, it's inverting a boolean
    And { block1: Condition, block2: Condition }, // Logical AND
    Or { block1: Condition, block2: Condition }, // Logical OR
    Touching { r#type: ParticleType }, // Looks neighbour to see if it's of type X
    If { condition: Condition, result: Action, r#else: Option<Action>}, // If block, if true, execute action
    OneInXChance { chance: NumberLiteral }, // Returns true one in a X chance, for example, if X is 3, it will return true 1/3 of the time
    IsEmpty { direction: Direction }, // Checks if a direction is empty
    CompareIs { block1: Numbers, block2: Numbers }, // Compares two blocks
    CompareBiggerThan { block1: Numbers, block2: Numbers }, // Compares two blocks
    CompareLessThan { block1: Numbers, block2: Numbers }, // Compares two blocks
    Boolean { value: bool }, // Returns a boolean value
}

// Implement from Block into Function
impl Blocks {
    #[allow(unused)]
    pub fn to_func(&self) -> Box<dyn Fn(&JSPlugin, Particle, &mut ParticleApi) -> bool> {
        let block = self.clone();
        match block {
            Blocks::Swap { direction } => {
                Box::new(move |plugin, particle, api| api.swap(direction[0], direction[1]))
            }
            Blocks::CopyTo { direction } => {
                Box::new(move |plugin, particle, api| api.set(direction[0], direction[1], particle))
            }
            Blocks::ChangeInto { direction, r#type } => Box::new(move |plugin, particle, api| {
                api.set(direction[0], direction[1], api.new_particle(r#type))
            }),
            Blocks::IfDirectionIsType { direction, r#type } => {
                Box::new(move |plugin, particle, api| {
                    api.get(direction[0], direction[1]).id == r#type
                })
            }
            Blocks::Not { block } => {
                Box::new(move |plugin, particle, api| !(block.to_func())(plugin, particle, api))
            }
            Blocks::And { block1, block2 } => Box::new(move |plugin, particle, api| {
                (block1.to_func())(plugin, particle, api)
                    && (block2.to_func())(plugin, particle, api)
            }),
            Blocks::Or { block1, block2 } => Box::new(move |plugin, particle, api| {
                (block1.to_func())(plugin, particle, api)
                    || (block2.to_func())(plugin, particle, api)
            }),
            Blocks::Touching { r#type } => Box::new(move |plugin, particle, api| {
                ParticleApi::NEIGHBORS
                    .iter()
                    .any(|(direction)| api.get_type(direction.x, direction.y) == r#type)
            }),
            Blocks::If {
                condition,
                result,
                r#else,
            } => {
                // We bake the functions here so they don't have to get built every time this block is called
                let condition = condition.to_func();
                let result = result.to_func();

                // "Baking" so we only call r#else if there is an else
                // I could also create an empty function with let r#else = r#else.unwrap_or_else(|| Box::new(|_, _, _| true));
                // And I could return a single lamnbda instead of this but....
                // These funcs might get called a lot, this is hotpath so I prefer to optimize as much as possible
                match r#else {
                    Some(r#else) => {
                        let r#else = r#else.to_func();
                        Box::new(move |plugin, particle, api| {
                            return if condition(plugin, particle, api) {
                                result(plugin, particle, api)
                            } else {
                                r#else(plugin, particle, api)
                            };
                        })
                    }
                    // By default, an if blocks returns true
                    None => Box::new(move |plugin, particle, api| {
                        return if condition(plugin, particle, api) {
                            result(plugin, particle, api)
                        } else {
                            true
                        };
                    }),
                }
            }
            Blocks::OneInXChance { chance } => {
                let chance = chance as i32;
                Box::new(move |plugin, particle, api| {
                    let random_number = api.gen_range(1, chance);
                    random_number == 0
                })
            }
            Blocks::CompareIs { block1, block2 } => Box::new(move |plugin, particle, api| {
                // It would be cool if I could precompute the numbers here
                // I shold be able to do for 2 of the variants, I could try to do that
                // and if it fails, that means I need to compute then insie the closure
                // Performance wise the impact should be minimal
                let number1 = block1.to_number(plugin, particle, api);
                let number2 = block2.to_number(plugin, particle, api);
                number1 == number2
            }),
            Blocks::CompareBiggerThan { block1, block2 } => {
                Box::new(move |plugin, particle, api| {
                    let number1 = block1.to_number(plugin, particle, api);
                    let number2 = block2.to_number(plugin, particle, api);
                    number1 > number2
                })
            }
            Blocks::CompareLessThan { block1, block2 } => Box::new(move |plugin, particle, api| {
                let number1 = block1.to_number(plugin, particle, api);
                let number2 = block2.to_number(plugin, particle, api);
                number1 < number2
            }),
            Blocks::Boolean { value } => Box::new(move |plugin, particle, api| value),
            Blocks::IsEmpty { direction } => {
                Box::new(move |plugin, particle, api| api.is_empty(direction[0], direction[1]))
            }
        }
    }
}
