// type Action = Box<dyn Fn(Particle, &mut ParticleApi)>;
type Action = Box<Blocks>;
type Direction = (i32, i32);
type ParticleType = u8;
type NumberLiteral = usize;

// Right now we assume users now the ID of the particles they want to check against
// Once we get this working, we are going to transform de json and substitute
// The particles names with an index that will be indexed into an array that has the particle IDs.

// Taken from Sandspiel Studio
#[derive(Debug)]
pub enum Blocks {
    Particle(ParticleType), // Particle type
    Swap(Direction),
    CopyTo(Direction),
    ChanteInto(Direction, ParticleType),
    IfDirecitonIsType(Direction, ParticleType), // If particle at direction is of type X
    Not(Box<Blocks>),           // Negates a block result, it's inverting a boolean
    And(Box<Blocks>, Box<Blocks>),   // Logical AND
    Or(Box<Blocks>, Box<Blocks>),    // Logical OR
    Touching(ParticleType),          // Looks neighbour to see if it's of type X
    NumberOfXTouching(ParticleType), // Counts number of neighbours of type X
    TypeOf(Direction),     // Returns type of particle at direction
    If(Box<Blocks>, Action),    // If block, if true, execute action
    OneInXChance(NumberLiteral), // Returns true one in a X chance, for example, if X is 3, it will return true 1/3 of the time
    CompareIs(Box<Blocks>, Box<Blocks>), // Compares two blocks
    CompareBiggerThan(Box<Blocks>, Box<Blocks>), // Compares two blocks
    CompareLessThan(Box<Blocks>, Box<Blocks>), // Compares two blocks
    Direction(Direction), // Returns a direction
    Boolean(bool),       // Returns a boolean value
}

// Is this redundant? Maybe, but it's better than writing strings in a match statement
// I could probably use a macro to generate this enum
pub enum BlockType {
    Particle,
    Swap,
    CopyTo,
    ChangeInto,
    IfDirectionIsType,
    Not,
    And,
    Or,
    Touching,
    NumberOfXTouching,
    TypeOf,
    If,
    OneInXChance,
    CompareIs,
    CompareBiggerThan,
    CompareLessThan,
    Direction,
    Boolean,
    INVALID,
}

impl From<&str> for BlockType {
    fn from(s: &str) -> Self {
        match s {
            "particle" => Self::Particle,
            "swap" => Self::Swap,
            "copyto" => Self::CopyTo,
            "changeinto" => Self::ChangeInto,
            "ifdirectionistype" => Self::IfDirectionIsType,
            "not" => Self::Not,
            "and" => Self::And,
            "or" => Self::Or,
            "touching" => Self::Touching,
            "numberofxtouching" => Self::NumberOfXTouching,
            "typeof" => Self::TypeOf,
            "if" => Self::If,
            "oneinxchance" => Self::OneInXChance,
            "compareis" => Self::CompareIs,
            "comparebiggerthan" => Self::CompareBiggerThan,
            "comparelessthan" => Self::CompareLessThan,
            "direction" => Self::Direction,
            "boolean" => Self::Boolean,
            _ => Self::INVALID,
        }
    }
}