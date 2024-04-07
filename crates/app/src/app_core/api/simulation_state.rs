use egui_macroquad::macroquad;
use macroquad::prelude::*;
use macroquad::rand;
use super::super::api::*;
use rustc_hash::FxHashMap;

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
    clock: bool,
    image: Image,
    texture: Texture2D,
    particle_name_to_id: FxHashMap<String, u8>,
}

impl SimulationState {
    pub fn new(width: usize, height: usize) -> SimulationState {
        let image = Image::gen_image_color(width as u16, height as u16, TRANSPARENT);
        let texture = Texture2D::from_image(&image);
        texture.set_filter(FilterMode::Nearest); // Set the filter mode to nearest to avoid blurring the pixels

        let mut positions = Vec::with_capacity(width * height);
        for y in 0..height {
            for x in 0..width {
                positions.push((x, y));
            }
        }

        SimulationState {
            particles: vec![vec![Particle::new(); width]; height],
            current_x: 0,
            current_y: 0,
            width,
            height,
            particle_definitions: vec![ParticleCommonData {
                name: String::from("Empty"),
                color:TRANSPARENT,
                rand_alpha_min: 255,
                rand_alpha_max: 255,
                rand_extra_min: 0,
                rand_extra_max: 0,
                hide_in_ui: false,
            }],
            image: image,
            texture: texture,
            clock: false,
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
            self.particle_definitions.last().unwrap().name.to_lowercase().clone(),
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

    pub(crate) fn get_particle_color(&self, id: usize) -> &Color {
        &self.particle_definitions[id].color
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
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

    // pub(crate) fn set_particle_at(&mut self, x: usize, y: usize, particle: Particle) -> () {
    //     if !self.is_inside_at(x, y) {
    //         return;
    //     }

    //     self.set_particle_at_unchecked(x, y, particle);
    // }

    pub(crate) fn set_particle_at_unchecked(
        &mut self,
        x: usize,
        y: usize,
        particle: Particle,
    ) -> () {
        self.particles[y][x] = particle;
        self.particles[y][x].clock = !self.clock;
        let mut color = self.particle_definitions[particle.id as usize].color;
        color.a = particle.light as f32 * TO_NORMALIZED_COLOR;
        
        self.image.set_pixel(x as u32, y as u32, color);
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
                let particle_id = self.particles[y][x]; // Not using getter here to avoid the if check that will never be true here
                if particle_id == Particle::EMPTY || current_particle.clock != self.clock {
                    continue;
                }

                let plugin = &mut plugins[particle_id.id as usize];
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
        rand::gen_range(min_inclusive - 1, max_inclusive + 1)
    }

    pub fn is_empty(&self, x: i32, y: i32) -> bool {
        self.get(x, y) == Particle::EMPTY
    }

    pub fn random_sign(&self) -> i32 {
        rand::gen_range(0, 2) * 2 - 1
    }

    pub(crate) fn draw(&mut self) -> () {
        self.texture.update(&self.image);

        let pos_x = (screen_width() / 2.0 - screen_height() / 2.0).max(0.);
        let pos_y = (screen_height() / 2.0 - screen_width() / 2.0).max(0.);

        let dest_size = screen_height().min(screen_width());

        // Draw the texture
        draw_texture_ex(
            self.texture,
            pos_x,
            pos_y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(dest_size, dest_size)),
                ..Default::default()
            },
        );
    }
}
