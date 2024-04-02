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
pub struct ParticleDefinition
{
    pub name: String,
    pub update_func: fn(&mut GameState, usize, usize) -> (),
    pub color: u32,

}

#[derive(Debug)]
pub struct GameState {
    buffer: Vec<u32>,
    pub particles: Vec<Vec<Particle>>,
    particle_definitions: Vec<ParticleDefinition>,
    current_x: usize,
    current_y: usize,
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
            particle_definitions: vec![
                ParticleDefinition {
                    name: String::from("empty"),
                    update_func: |_, _, _| {}, // Función vacía
                    color: 0,
                }
            ],
        }
    }

    pub fn get_particle_definitions(&self) -> &Vec<ParticleDefinition> {
        &self.particle_definitions
    }

    pub fn add_particle_definition(&mut self, particle_definition: ParticleDefinition) -> () {
        self.particle_definitions.push(particle_definition);

        // Print the name of the particle definition
        println!("Added particle definition: {}", self.particle_definitions.last().unwrap().name);
    }

    pub fn set_particle(&mut self, x: usize, y: usize, id: u32) -> () {
        
        if x >= self.width || y >= self.height || x < 0 || y < 0{
            return;
        }
        self.particles[y][x] = Particle { id: id };
    }

    pub fn get_particle_id(&self, x: usize, y: usize) -> u32 {
        self.particles[y][x].id
    }

    pub fn get_buffer(&self) -> &Vec<u32> {
        &self.buffer
    }

    pub fn update(&mut self) -> () {
        for y in 0..self.height {
            for x in 0..self.width {
                self.current_x = x;
                self.current_y = y;
               
                (self.particle_definitions[self.get_particle_id(x,y) as usize].update_func)(self, x, y);
            }
        }
    }

    pub fn draw(&mut self) -> () {
        for y in 0..self.height {
            for x in 0..self.width {
                self.buffer[(x + y * self.width as usize) as usize] = self.particle_definitions[self.particles[y][x].id as usize].color;
            }
        }
    }
}

pub trait Plugin {
    fn register(&mut self) -> ParticleDefinition;
}
