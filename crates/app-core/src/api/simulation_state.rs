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
}

impl SimulationState {
    pub fn new(width: usize, height: usize) -> SimulationState {
        #[rustfmt::skip]
        let not_black_color = [Color::NOT_BLACK.r, Color::NOT_BLACK.g, Color::NOT_BLACK.b, Color::NOT_BLACK.a];

        let mut color_buffer = vec![0; width * height * 4];
        for (_, color) in color_buffer.chunks_mut(4).enumerate() {
            color.copy_from_slice(&not_black_color);
        }


        SimulationState {
            particles: vec![vec![Particle::new(); width]; height],
            current_x: 0,
            current_y: 0,
            width,
            height,
            particle_definitions: vec![ParticleCommonData {
                name: String::from("Empty"),
                color: [18, 33, 43, 1],
                rand_alpha_min: 0,
                rand_alpha_max: 0,
                rand_extra_min: 0,
                rand_extra_max: 0,
                hide_in_ui: false,
            }],
            color_buffer,
            clock: 0,
            particle_name_to_id: FxHashMap::default(),
        }
    }

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

    pub fn id_from_name(&self, name: &str) -> u8 {
        *self
            .particle_name_to_id
            .get(&name.to_lowercase())
            .unwrap_or(&Particle::INVALID.id)
    }

    pub(crate) fn add_particle_definition(
        &mut self,
        particle_definition: ParticleCommonData,
    ) -> () {
        self.particle_definitions.push(particle_definition);
        self.particle_name_to_id.insert(
            self.particle_definitions
                .last()
                .unwrap()
                .name
                .to_lowercase()
                .clone(),
            self.particle_definitions.len() as u8 - 1,
        );

        // Print the name of the particle definition
        println!(
            "Added particle definition: {}",
            self.particle_definitions.last().unwrap().name
        );
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

    pub fn get(&self, x: i32, y: i32) -> Particle {
        let local_x = (self.current_x as i32 - x) as usize;
        let local_y = (self.current_y as i32 - y) as usize;

        if !self.is_inside_at(local_x, local_y) {
            return Particle::INVALID;
        }

        self.particles[local_y][local_x]
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
        self.color_buffer[start_index + 3] = particle.light;
    }

    pub fn set(&mut self, x: i32, y: i32, particle: Particle) -> bool {
        let local_x = (self.current_x as i32 - x) as usize;
        let local_y = (self.current_y as i32 - y) as usize;

        if !self.is_inside_at(local_x, local_y) {
            return false;
        }

        self.set_particle_at_unchecked(local_x, local_y, particle);
        true
    }

    pub fn is_inside(&self, x: i32, y: i32) -> bool {
        let local_x = (self.current_x as i32 - x) as usize;
        let local_y = (self.current_y as i32 - y) as usize;

        self.is_inside_at(local_x, local_y)
    }

    pub fn move_to(&mut self, x: i32, y: i32) -> bool {
        let local_x = (self.current_x as i32 - x) as usize;
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
        let local_x = (self.current_x as i32 - x) as usize;
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
        let local_x = (self.current_x as i32 - x) as usize;
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
        let local_x = (self.current_x as i32 - x) as usize;
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
                let current_particle = &self.particles[y][x];
                if current_particle.id == Particle::EMPTY.id || current_particle.clock != self.clock
                {
                    continue;
                }

                let plugin = &mut plugins[current_particle.id as usize];
                plugin.update(self.particles[y][x], self);
            }
        }

        // Post update
        for plugin in plugins.iter_mut() {
            plugin.post_update(self);
        }

        self.current_x = 0;
        self.current_y = 0;
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
}
