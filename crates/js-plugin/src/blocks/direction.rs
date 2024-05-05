use super::*;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged, rename_all = "camelCase")]
pub enum Direction {
    Constant([i32; 2]),
    Random,
    Operation(MathOperations, Box<Direction>, Box<Direction>),
}

impl Direction {
    pub fn get_direction(&self, api: &ParticleApi) -> [i32; 2] {
        match self {
            Direction::Constant(direction) => *direction,
            Direction::Random => [api.gen_range(-1, 1), api.gen_range(-1, 1)],
            Direction::Operation(op, dir1, dir2) => {
                let dir1 = dir1.get_direction(api);
                let dir2 = dir2.get_direction(api);
                match op {
                    MathOperations::Addition => [dir1[0] + dir2[0], dir1[1] + dir2[1]],
                    MathOperations::Subtraction => [dir1[0] - dir2[0], dir1[1] - dir2[1]],
                    MathOperations::Multiplication => [dir1[0] * dir2[0], dir1[1] * dir2[1]],
                    MathOperations::Division => [dir1[0] / dir2[0], dir1[1] / dir2[1]],
                    MathOperations::Modulo => [dir1[0] % dir2[0], dir1[1] % dir2[1]],
                    MathOperations::Difference => [
                        (dir1[0] - dir2[0]).abs(),
                        (dir1[1] - dir2[1]).abs(),
                    ],
                }
            }
        }
    }
}
