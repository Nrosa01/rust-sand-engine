use crate::api::*;
use rustc_hash::FxHashMap;

use std::println;
use std::vec;

pub struct Vec2i {
    pub x: i32,
    pub y: i32,
}

pub struct Vec2u {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Transformation {
    HorizontalReflection(bool),
    VerticalReflection(bool),
    Reflection(bool, bool),
    Rotation(usize),
    None,
}

impl Transformation {
    pub fn transform(&self, direction: &[i32; 2]) -> [i32; 2] {
        match self {
            Transformation::HorizontalReflection(true) => [-direction[0], direction[1]],
            Transformation::VerticalReflection(true) => [direction[0], -direction[1]],
            Transformation::Reflection(true, true) => [-direction[0], -direction[1]],
            Transformation::Reflection(true, false) => [-direction[0], direction[1]],
            Transformation::Reflection(false, true) => [direction[0], -direction[1]],
            // Rotation mus be a number between 0 and 7
            Transformation::Rotation(rotations) => match direction {
                [0, 1] => SimulationState::DIRECTIONS_VEC[*rotations],
                [1, 1] => SimulationState::DIRECTIONS_VEC[*rotations + 1],
                [1, 0] => SimulationState::DIRECTIONS_VEC[*rotations + 2],
                [1, -1] => SimulationState::DIRECTIONS_VEC[*rotations + 3],
                [0, -1] => SimulationState::DIRECTIONS_VEC[*rotations + 4],
                [-1, -1] => SimulationState::DIRECTIONS_VEC[*rotations + 5],
                [-1, 0] => SimulationState::DIRECTIONS_VEC[*rotations + 6],
                [-1, 1] => SimulationState::DIRECTIONS_VEC[*rotations + 7],
                _ => direction.clone(),
            },
            Transformation::None => direction.clone(),
            _ => direction.clone(),
        }
    }
}

pub struct SimulationState {
    particle_definitions: Vec<ParticleCommonData>,
    particles: Vec<Vec<Particle>>,
    current_x: usize,
    current_y: usize,
    width: usize,
    height: usize,
    clock: u8,
    color_buffer: Vec<u8>,
    particle_name_to_id: FxHashMap<String, u8>,
    transformation: Transformation,
    frame_count: u32,
}

impl SimulationState {
    pub fn new(width: usize, height: usize) -> SimulationState {
        let color_buffer = vec![0; width * height * 4];

        let mut state = SimulationState {
            particles: vec![vec![Particle::new(); width]; height],
            current_x: 0,
            current_y: 0,
            width,
            height,
            particle_definitions: Vec::new(),
            color_buffer,
            clock: 0,
            particle_name_to_id: FxHashMap::default(),
            transformation: Transformation::None,
            frame_count: 0,
        };

        state.add_or_replace_particle_definition(ParticleCommonData {
            name: String::from("Empty"),
            color:  [204, 225, 251, 255],
            rand_alpha_min: 1,
            rand_alpha_max: 1,
            rand_extra_min: 0,
            rand_extra_max: 0,
        });

        state
    }

    #[allow(unused)]
    const DIRECTIONS_VEC: [[i32; 2]; 16] = [
        [0, 1],   // N 0
        [1, 1],   // NE 1
        [1, 0],   // E 2
        [1, -1],  // SE 3
        [0, -1],  // S 4
        [-1, -1], // SW 5
        [-1, 0],  // W 6
        [-1, 1],  // NW 7
        [0, 1],   // N 0
        [1, 1],   // NE 1
        [1, 0],   // E 2
        [1, -1],  // SE 3
        [0, -1],  // S 4
        [-1, -1], // SW 5
        [-1, 0],  // W 6
        [-1, 1],  // NW 7
    ];

    pub const NEIGHBORS: [Vec2i; 8] = [
        Vec2i { x: 0, y: -1 },
        Vec2i { x: 1, y: -1 },
        Vec2i { x: 1, y: 0 },
        Vec2i { x: 1, y: 1 },
        Vec2i { x: 0, y: 1 },
        Vec2i { x: -1, y: 1 },
        Vec2i { x: -1, y: 0 },
        Vec2i { x: -1, y: -1 },
    ];

    pub const NEIGHBORS_CROSS: [Vec2i; 4] = [
        Vec2i { x: 0, y: -1 },
        Vec2i { x: 1, y: 0 },
        Vec2i { x: 0, y: 1 },
        Vec2i { x: -1, y: 0 },
    ];

    pub const NEIGHBORS_DIAGONAL: [Vec2i; 4] = [
        Vec2i { x: 1, y: -1 },
        Vec2i { x: 1, y: 1 },
        Vec2i { x: -1, y: 1 },
        Vec2i { x: -1, y: -1 },
    ];

    pub fn set_transformation(&mut self, transformation: Transformation) -> () {
        self.transformation = transformation;
    }

    pub fn get_transformation(&self) -> &Transformation {
        &self.transformation
    }

    pub fn id_from_name(&self, name: &str) -> u8 {
        *self
            .particle_name_to_id
            .get(&name.to_lowercase())
            .unwrap_or(&Particle::INVALID.id)
    }

    // Returns Some(id) if particle was updated None if it was added
    pub(crate) fn add_or_replace_particle_definition(&mut self, particle_definition: ParticleCommonData) -> Option<usize>{
        let name = particle_definition.name.to_lowercase();

        if self.particle_name_to_id.contains_key(&name) {
            let id = *self.particle_name_to_id.get(&name).unwrap();
            self.particle_definitions[id as usize] = particle_definition;
            Some(id as usize)
        } else {
            self.particle_definitions.push(particle_definition);
            self.particle_name_to_id
                .insert(name.to_lowercase().clone(), (self.particle_definitions.len() - 1) as u8);

            println!("Added or updated particle definition: {}", name);
            None
        }
    }

    pub(crate) fn remove_particle_definition(&mut self, id: u8) -> () {
        let name = self.particle_definitions[id as usize].name.to_lowercase();
        self.particle_name_to_id.remove(&name);
        self.particle_definitions.remove(id as usize);

        // Now, values in the map might be invalidated, every value higher than the removed one will be shifted (decreased by 1)
        self.particle_name_to_id.iter_mut().for_each(|(_, value)| {
            if *value > id {
                *value -= 1;
            }
        });


        // We have to update the particle buffer as indices higher than the removed one will be shifted
        // If the index is the same as the removed one we'll just replace it with an empty particle
        for y in 0..self.height {
            for x in 0..self.width {
                let particle = &mut self.particles[y][x];
                if particle.id == id as u8 {
                    *particle = Particle::EMPTY;
                } else if particle.id > id as u8 {
                    particle.id -= 1;
                }
            }
        }
    }

    pub(crate) fn get_particle_definitions(&self) -> &Vec<ParticleCommonData> {
        &self.particle_definitions
    }

    pub(crate) fn get_particle_name(&self, id: usize) -> &String {
        &self.particle_definitions[id].name
    }

    pub(crate) fn get_particle_color(&self, id: usize) -> &[u8; 4] {
        &self.particle_definitions[id].color
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get_buffer(&self) -> &[u8] {
        &self.color_buffer
    }

    pub fn get_current(&self) -> Particle {
        self.particles[self.current_y][self.current_x]
    }

    pub fn get_current_mut(&mut self) -> &mut Particle {
        &mut self.particles[self.current_y][self.current_x]
    }

    pub fn get_mut(&mut self, x: i32, y: i32) -> Option<&mut Particle> {
        let local_x = (self.current_x as i32 + x) as usize;
        let local_y = (self.current_y as i32 - y) as usize;

        if !self.is_inside_at(local_x, local_y) {
            return None;
        }

        Some(&mut self.particles[local_y][local_x])
    }

    pub fn get(&self, x: i32, y: i32) -> Particle {
        let local_x = (self.current_x as i32 + x) as usize;
        let local_y = (self.current_y as i32 - y) as usize;

        if !self.is_inside_at(local_x, local_y) {
            return Particle::INVALID;
        }

        self.particles[local_y][local_x]
    }

    pub fn get_particle_count(&self) -> u8 {
        self.particle_definitions.len() as u8
    }

    pub fn new_particle(&self, particle_id: u8) -> Particle {
        let particle_definition = &self.particle_definitions[particle_id as usize];
        let min_alpha = particle_definition.rand_alpha_min as i32;
        let max_alpha = particle_definition.rand_alpha_max as i32;
        let extra_min = particle_definition.rand_extra_min as i32;
        let extra_max = particle_definition.rand_extra_max as i32;

        Particle {
            id: particle_id,
            light: self.gen_range(min_alpha, max_alpha) as u8,
            clock: !self.clock,
            extra: self.gen_range(extra_min, extra_max) as u8,
        }
    }

    pub(crate) fn set_particle_at_by_id(&mut self, x: usize, y: usize, particle_id: u8) -> () {
        if !self.is_inside_at(x, y) {
            return;
        }

        self.set_particle_at_unchecked(x, y, self.new_particle(particle_id));
    }

    pub(crate) fn set_particle_at_unchecked(
        &mut self,
        x: usize,
        y: usize,
        particle: Particle,
    ) -> () {
        self.particles[y][x].id = particle.id;
        self.particles[y][x].light = particle.light;
        self.particles[y][x].extra = particle.extra;
        self.particles[y][x].clock = !self.clock;
        let color = self.particle_definitions[particle.id as usize].color;

        // This was better to read, but it uses unsafe code under the hood
        // self.image.get_image_data_mut()[y * self.width + x] = [255,255,255,255];

        // Wasm lto optimizes this better than the above
        let start_index = (y * self.width + x) * 4;
        self.color_buffer[start_index] = color[0];
        self.color_buffer[start_index + 1] = color[1];
        self.color_buffer[start_index + 2] = color[2];
        self.color_buffer[start_index + 3] = ((particle.light as u16 * 255) / 100) as u8;
        // self.color_buffer[start_index + 3] = particle.light;
    }

    pub fn get_particles(&self) -> &Vec<Vec<Particle>> {
        &self.particles
    }

    pub fn set(&mut self, x: i32, y: i32, particle: Particle) -> bool {
        let local_x = (self.current_x as i32 + x) as usize;
        let local_y = (self.current_y as i32 - y) as usize;

        if !self.is_inside_at(local_x, local_y) {
            return false;
        }

        self.set_particle_at_unchecked(local_x, local_y, particle);
        true
    }

    pub fn is_inside(&self, x: i32, y: i32) -> bool {
        let local_x = (self.current_x as i32 + x) as usize;
        let local_y = (self.current_y as i32 - y) as usize;

        self.is_inside_at(local_x, local_y)
    }

    pub fn move_to(&mut self, x: i32, y: i32) -> bool {
        let local_x = (self.current_x as i32 + x) as usize;
        let local_y = (self.current_y as i32 - y) as usize;

        if !self.is_inside_at(local_x, local_y) {
            return false;
        }

        let particle = self.particles[self.current_y][self.current_x];
        self.set_particle_at_unchecked(local_x, local_y, particle);
        self.set_particle_at_unchecked(self.current_x, self.current_y, Particle::EMPTY);

        self.current_x = local_x;
        self.current_y = local_y;
        true
    }

    pub fn move_to_using(&mut self, x: i32, y: i32, particle: Particle) -> bool {
        let local_x = (self.current_x as i32 + x) as usize;
        let local_y = (self.current_y as i32 - y) as usize;

        if !self.is_inside_at(local_x, local_y) {
            return false;
        }

        self.set_particle_at_unchecked(local_x, local_y, particle);
        self.set_particle_at_unchecked(self.current_x, self.current_y, Particle::EMPTY);

        self.current_x = local_x;
        self.current_y = local_y;
        true
    }

    pub fn swap(&mut self, x: i32, y: i32) -> bool {
        let local_x = (self.current_x as i32 + x) as usize;
        let local_y = (self.current_y as i32 - y) as usize;

        if !self.is_inside_at(local_x, local_y) {
            return false;
        }

        let swap_particle = self.particles[local_y][local_x];
        let particle = self.particles[self.current_y][self.current_x];
        self.set_particle_at_unchecked(self.current_x, self.current_y, swap_particle);
        self.set_particle_at_unchecked(local_x, local_y, particle);

        self.current_x = local_x;
        self.current_y = local_y;
        true
    }

    pub fn swap_using(&mut self, x: i32, y: i32, particle: Particle) -> bool {
        let local_x = (self.current_x as i32 + x) as usize;
        let local_y = (self.current_y as i32 - y) as usize;

        if !self.is_inside_at(local_x, local_y) {
            return false;
        }

        let swap_particle = self.particles[local_y][local_x];
        self.set_particle_at_unchecked(local_x, local_y, particle);
        self.set_particle_at_unchecked(self.current_x, self.current_y, swap_particle);

        self.current_x = local_x;
        self.current_y = local_y;
        true
    }

    pub fn is_particle_at(&self, x: i32, y: i32, particle_id: u8) -> bool {
        self.get(x, y) == particle_id
    }

    pub fn is_any_particle_at(&self, x: i32, y: i32, particles: &[u8]) -> bool {
        let particle = self.get(x, y);
        particles.contains(&particle.id)
    }

    pub(crate) fn is_inside_at(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    pub(super) fn update(
        &mut self,
        plugins: &mut Vec<Box<dyn Plugin>>,
        order_scheme: &OrderScheme,
    ) -> () {
        self.clock = !self.clock;

        for y in order_scheme.order_y {
            for x in order_scheme.order_x {
                let x = x as usize;
                let y = y as usize;

                self.current_x = x;
                self.current_y = y;
                let current_particle = self.particles[y][x];
                if current_particle.clock != self.clock
                {
                    continue;
                }

                let plugin = &mut plugins[current_particle.id as usize];
                plugin.update(self);
            }
        }

        self.current_x = 0;
        self.current_y = 0;
        self.frame_count += 1;
    }

    pub fn get_frame(&self) -> u32 {
        self.frame_count
    }

    /// Range, min and max are inclusive
    pub fn gen_range(&self, min_inclusive: i32, max_inclusive: i32) -> i32 {
        fastrand::i32(min_inclusive..=max_inclusive)
    }

    pub fn is_empty(&self, x: i32, y: i32) -> bool {
        self.get(x, y) == Particle::EMPTY
    }

    pub fn random_sign(&self) -> i32 {
        fastrand::i32(0..2) * 2 - 1
    }

    pub fn random_bool(&self) -> bool {
        fastrand::bool()
    }

    pub fn get_type(&self, x: i32, y: i32) -> u8 {
        self.get(x, y).id
    }

    pub fn clear(&mut self) -> () {
        for y in 0..self.height {
            for x in 0..self.width {
                self.set_particle_at_unchecked(x, y, Particle::EMPTY);
            }
        }
    }

    pub fn resize(&mut self, size: u32) {
        let current_size = self.width as u32;

        self.width = size as usize;
        self.height = size as usize;

        let mut new_particles: Vec<Vec<Particle>> =
            vec![vec![Particle::EMPTY; size as usize]; size as usize];

        let offset = ((size as i32) - (current_size as i32)) / 2;

        // I want to make this feel like a "crop", so if a Particle is in the midddle, it will stay in the midle when you resize
        // Particle position is their position in the array, but now the array changed so we have to adapt it
        // Sadly this is not an in-place solution like using vector::resize, but this function is called very sparingly
        // so it's fine to keep it like this.
        for (y, row) in self.particles.iter().enumerate() {
            for (x, particle) in row.iter().enumerate() {
                let new_x = x as i32 + offset;
                let new_y = y as i32 + offset;
                if new_x >= 0 && new_x < size as i32 && new_y >= 0 && new_y < size as i32 {
                    new_particles[new_y as usize][new_x as usize] = *particle;
                }
            }
        }

        self.particles = new_particles;

        let color_buffer_size = (size * size * 4) as usize;
        self.color_buffer
            .resize(color_buffer_size, Default::default());

        // As buffer is a linear vector and particles a 2d matrix, we can't be sure color buffer state is correct
        // So for now I will just repaint each particle
        self.repaint();
    }

    pub fn repaint(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let particle = &self.particles[y][x];
                self.set_particle_at_unchecked(x, y, *particle);
            }
        }
    }
}
