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
    Particle(ParticleType), // Particle type
    Swap{ direction: Direction },
    CopyTo(Direction),
    ChanteInto(Direction, ParticleType),
    IfDirectionIsType{direction: Direction, r#type: ParticleType}, // If particle at direction is of type X
    Not(Box<Blocks>),           // Negates a block result, it's inverting a boolean
    And(Box<Blocks>, Box<Blocks>),   // Logical AND
    Or(Box<Blocks>, Box<Blocks>),    // Logical OR
    Touching(ParticleType),          // Looks neighbour to see if it's of type X
    NumberOfXTouching(ParticleType), // Counts number of neighbours of type X
    TypeOf(Direction),     // Returns type of particle at direction
    If{condition: Box<Blocks>, result: Action},    // If block, if true, execute action
    OneInXChance(NumberLiteral), // Returns true one in a X chance, for example, if X is 3, it will return true 1/3 of the time
    CompareIs(Box<Blocks>, Box<Blocks>), // Compares two blocks
    CompareBiggerThan(Box<Blocks>, Box<Blocks>), // Compares two blocks
    CompareLessThan(Box<Blocks>, Box<Blocks>), // Compares two blocks
    Direction(Direction), // Returns a direction
    Boolean(bool),       // Returns a boolean value
}