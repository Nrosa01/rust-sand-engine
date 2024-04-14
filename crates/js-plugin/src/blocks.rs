use app_core::{Particle, ParticleApi};
use serde::{Deserialize, Serialize};

// type Action = Box<dyn Fn(Particle, &mut ParticleApi)>;
type Action = Box<Blocks>;
type Direction = [i32; 2];
type ParticleType = u8;
type NumberLiteral = usize;

// Right now we assume users now the ID of the particles they want to check against
// Once we get this working, we are going to transform de json and substitute
// The particles names with an index that will be indexed into an array that has the particle IDs.


// TODO: Update self x and self y in swap function on ParticleApi

// Taken from Sandspiel Studio
#[derive(Debug,Serialize,Deserialize, Clone)]
#[serde(tag = "block", content = "data", rename_all = "camelCase")]
pub enum Blocks {
    Particle { r#type: ParticleType }, // Particle type
    Swap { direction: Direction },
    CopyTo { direction: Direction },
    ChangeInto { direction: Direction, r#type: ParticleType },
    IfDirectionIsType { direction: Direction, r#type: ParticleType }, // If particle at direction is of type X
    Not { block: Box<Blocks> },           // Negates a block result, it's inverting a boolean
    And { block1: Box<Blocks>, block2: Box<Blocks> },   // Logical AND
    Or { block1: Box<Blocks>, block2: Box<Blocks> },    // Logical OR
    Touching { r#type: ParticleType },          // Looks neighbour to see if it's of type X
    NumberOfXTouching { r#type: ParticleType }, // Counts number of neighbours of type X
    TypeOf { direction: Direction },     // Returns type of particle at direction
    If { condition: Box<Blocks>, result: Action },    // If block, if true, execute action
    OneInXChance { chance: NumberLiteral }, // Returns true one in a X chance, for example, if X is 3, it will return true 1/3 of the time
    CompareIs { block1: Box<Blocks>, block2: Box<Blocks> }, // Compares two blocks
    CompareBiggerThan { block1: Box<Blocks>, block2: Box<Blocks> }, // Compares two blocks
    CompareLessThan { block1: Box<Blocks>, block2: Box<Blocks> }, // Compares two blocks
    Direction { direction: Direction }, // Returns a direction
    Boolean { value: bool },       // Returns a boolean value
}

// Implement from Block into Function
impl Blocks {
    #[allow(unused)]
    pub fn to_func(&self) -> Box<dyn Fn(Particle, &mut ParticleApi) -> bool> {
        let block = self.clone();
        match block {
            Blocks::Particle { r#type } => Box::new(move |particle, api| {
                true
            }),
            Blocks::Swap { direction } => Box::new(move |particle, api| {
                api.swap(direction[0], direction[1])
            }),
            Blocks::CopyTo { direction } => Box::new(move |particle, api| {
                api.set(direction[0], direction[1], particle)
            }),
            Blocks::ChangeInto { direction, r#type } => Box::new(move |particle, api| {
                api.set(direction[0], direction[1], api.new_particle(r#type))
            }),
            Blocks::IfDirectionIsType { direction, r#type } => Box::new(move |particle, api| {
                api.get(direction[0], direction[1]).id == r#type
            }),
            Blocks::Not { block } => Box::new(move |particle, api| {
                !(block.to_func())(particle, api)
            }),
            Blocks::And { block1, block2 } => Box::new(move |particle, api| {
                (block1.to_func())(particle, api) && (block2.to_func())(particle, api)
            }),
            Blocks::Or { block1, block2 } => Box::new(move |particle, api| {
                (block1.to_func())(particle, api) || (block2.to_func())(particle, api)
            }),
            Blocks::Touching { r#type } => Box::new(move |particle, api| {
                true
            }),
            Blocks::NumberOfXTouching { r#type } => Box::new(move |particle, api| {
                true
            }),
            Blocks::TypeOf { direction } => Box::new(move |particle, api| {
                true
            }),
            Blocks::If { condition, result } => Box::new(move |particle, api| {
                if (condition.to_func())(particle, api) {
                    return (result.to_func())(particle, api)
                }
                true
            }),
            Blocks::OneInXChance { chance } => Box::new(move |particle, api| {
                true
            }),
            Blocks::CompareIs { block1, block2 } => Box::new(move |particle, api| {
                true
            }),
            Blocks::CompareBiggerThan { block1, block2 } => Box::new(move |particle, api| {
                true
            }),
            Blocks::CompareLessThan { block1, block2 } => Box::new(move |particle, api| {
                true
            }),
            Blocks::Direction { direction } => Box::new(move |particle, api| {
                true
            }),
            Blocks::Boolean { value } => Box::new(move |particle, api| {
                true
            }),
        }
    }
}