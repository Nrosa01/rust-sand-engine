use std::fmt::Display;

#[derive(Debug)]
pub struct Error {
    message: String,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for Error {}

#[derive(Clone, Debug)]
pub struct Particle {
    pub id: u32,
}

#[derive(Debug)]
pub struct GameState {
    buffer: Vec<u32>,
    pub particles: Vec<Vec<Particle>>,
    pub current_x: usize,
    pub current_y: usize,
    pub width: usize,
    pub height: usize,
}

impl GameState {
    pub fn new(width: usize, height: usize) -> GameState {
        GameState {
            buffer: vec![0; width * height],
            particles: vec![vec![Particle { id: 0 }; width]; height],
            current_x: 0,
            current_y: 0,
            width,
            height,
        }
    }

    pub fn set_particle(&mut self, x: usize, y: usize, id: u32) -> () {
        self.particles[y][x] = Particle { id: id };
    }

    pub fn get_particle_id(&self, x: usize, y: usize) -> u32 {
        self.particles[y][x].id
    }

    pub fn get_buffer(&self) -> &Vec<u32> {
        &self.buffer
    }

    pub fn draw(&mut self) -> () {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.particles[y][x].id == 1 {
                    self.buffer[(x + y * self.width as usize) as usize] = 0xFFFF00;
                } else {
                    self.buffer[(x + y * self.width as usize) as usize] = 0x000000;
                }
            }
        }
    }
}
pub trait Plugin {
    fn hook(&mut self, context: &mut GameState) -> Result<(), Error>;
}
