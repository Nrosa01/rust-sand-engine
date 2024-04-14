use serde::{Deserialize, Serialize};

// type Action = Box<dyn Fn(Particle, &mut ParticleApi)>;
type Action = Box<Blocks>;
type Direction = [i32; 2];
type ParticleType = u8;
type NumberLiteral = usize;

// Right now we assume users now the ID of the particles they want to check against
// Once we get this working, we are going to transform de json and substitute
// The particles names with an index that will be indexed into an array that has the particle IDs.

// Taken from Sandspiel Studio
#[derive(Debug,Serialize,Deserialize)]
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