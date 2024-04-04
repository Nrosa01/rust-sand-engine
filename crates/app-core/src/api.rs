use crate::Plugin;
use macroquad::prelude::*;
use rustc_hash::FxHashMap;
use std::vec;

pub const TO_NORMALIZED_COLOR: f32 = 1.0 / 255.0;

pub struct Empty;

impl Plugin for Empty {
    fn register(&mut self) -> ParticleCommonData {
        ParticleCommonData {
            name: String::from("Empty"),
            color: BLACK,
        }
    }

    fn update(&self, _cell: Particle, _api: &mut SimulationState) {}
}

#[derive(Clone, Debug, Copy)]
pub struct Particle {
    pub id: u8,
    pub clock: bool,
    pub light: u8,
    pub extra: u8,
}

impl Particle {
    pub fn new() -> Particle {
        Particle {
            id: 0,
            clock: false,
            light: rand::gen_range(-1, 256) as u8,
            extra: 0,
        }
    }

    pub fn from_id(id: u8) -> Particle {
        //print something
        Particle {
            id,
            clock: false,
            light: rand::gen_range(-1, 256) as u8,
            extra: 0,
        }
    }

    pub const EMPTY: Particle = Particle {
        id: 0,
        clock: false,
        light: 0,
        extra: 0,
    };

    pub(crate) const INVALID: Particle = Particle {
        id: u8::MAX,
        clock: false,
        light: 0,
        extra: 0,
    };
}

impl PartialEq for Particle {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl PartialEq<usize> for Particle {
    fn eq(&self, other: &usize) -> bool {
        self.id == *other as u8
    }
}

impl PartialEq<u8> for Particle {
    fn eq(&self, other: &u8) -> bool {
        self.id == *other
    }
}

impl From<usize> for Particle {
    fn from(id: usize) -> Self {
        Particle::from_id(id as u8)
    }
}

impl From<u8> for Particle {
    fn from(id: u8) -> Self {
        Particle::from_id(id as u8)
    }
}

#[derive(Debug)]
pub struct ParticleCommonData {
    pub name: String,
    pub color: Color,
}

pub struct PluginData {
    pub(crate) plugins: Vec<Box<dyn Plugin>>,
    pub(crate) libraries: Vec<libloading::Library>,
}

impl Drop for PluginData {
    fn drop(&mut self) {
        // Drop first plugins, then libraries
        for plugin in self.plugins.drain(..) {
            drop(plugin);
        }

        for library in self.libraries.drain(..) {
            drop(library);
        }
    }
}

impl PluginData {
    pub fn new() -> PluginData {
        PluginData {
            libraries: Vec::new(),
            plugins: vec![Box::new(Empty)],
        }
    }
}

#[derive(Copy, Clone)]
pub(crate) struct CustomRange {
    current: isize,
    end: isize,
    step: isize,
}

impl CustomRange {
    pub fn new(start: isize, end: isize, step: isize) -> CustomRange {
        CustomRange {
            current: start,
            end,
            step,
        }
    }
}

impl Iterator for CustomRange {
    type Item = isize;

    fn next(&mut self) -> Option<Self::Item> {
        if (self.step > 0 && self.current < self.end) || (self.step < 0 && self.current > self.end)
        {
            let result = self.current;
            self.current += self.step;
            Some(result)
        } else {
            None
        }
    }
}

pub(crate) struct OrderScheme {
    order_x: CustomRange,
    order_y: CustomRange,
}

impl OrderScheme {
    pub fn new(order_x: CustomRange, order_y: CustomRange) -> OrderScheme {
        OrderScheme {
            order_x: order_x,
            order_y: order_y,
        }
    }
}

struct OrderSchemes {
    ltr_ttb: OrderScheme,
    ltr_btt: OrderScheme,
    rtl_ttb: OrderScheme,
    rtl_btt: OrderScheme,
    current: usize,
}

impl OrderSchemes {
    pub fn new(width: usize, height: usize) -> OrderSchemes {
        OrderSchemes {
            ltr_ttb: OrderScheme::new(
                CustomRange::new(0, width as isize, 1),
                CustomRange::new(0, height as isize, 1),
            ),
            ltr_btt: OrderScheme::new(
                CustomRange::new(0, width as isize, 1),
                CustomRange::new((height - 1) as isize, -1, -1),
            ),
            rtl_ttb: OrderScheme::new(
                CustomRange::new((width - 1) as isize, -1, -1),
                CustomRange::new(0, height as isize, 1),
            ),
            rtl_btt: OrderScheme::new(
                CustomRange::new((width - 1) as isize, -1, -1),
                CustomRange::new((height - 1) as isize, -1, -1),
            ),
            current: 0,
        }
    }

    pub fn get_ciclying(&mut self) -> &OrderScheme {
        let scheme = match self.current {
            0 => &self.ltr_ttb,
            1 => &self.rtl_ttb,
            2 => &self.rtl_btt,
            3 => &self.ltr_btt,
            _ => &self.ltr_ttb,
        };

        self.current = (self.current + 1) % 4;
        scheme
    }
}

pub struct Simulation {
    simulation_state: SimulationState,
    plugin_data: PluginData,
    order_scheme: OrderSchemes,
}

type PluginLoader<'a> =
    Result<libloading::Symbol<'a, fn() -> Vec<Box<dyn Plugin>>>, libloading::Error>;

impl Simulation {
    pub fn new(width: usize, height: usize) -> Simulation {
        Simulation {
            simulation_state: SimulationState::new(width, height),
            plugin_data: PluginData::new(),
            order_scheme: OrderSchemes::new(width, height),
        }
    }

    pub fn get_state(&self) -> &SimulationState {
        &self.simulation_state
    }

    pub fn get_state_mut(&mut self) -> &mut SimulationState {
        &mut self.simulation_state
    }

    pub fn get_plugin_count(&self) -> usize {
        self.plugin_data.plugins.len()
    }

    pub fn get_width(&self) -> usize {
        self.simulation_state.width
    }

    pub fn get_height(&self) -> usize {
        self.simulation_state.height
    }

    pub fn update(&mut self) -> () {
        self.simulation_state.update(
            &mut self.plugin_data.plugins,
            &self.order_scheme.get_ciclying(),
        );
    }

    pub fn draw(&mut self) -> () {
        self.simulation_state.draw();
    }

    pub fn get_particle_name(&self, id: usize) -> Result<&String, String> {
        if id >= self.get_plugin_count() {
            return Err(format!("Particle with id {} not found", id));
        }

        Ok(&self.simulation_state.get_particle_name(id))
    }

    pub fn get_particle_color(&self, id: usize) -> Result<&Color, String> {
        if id >= self.get_plugin_count() {
            return Err(format!("Particle with id {} not found", id));
        }

        Ok(&self.simulation_state.get_particle_color(id))
    }

    pub fn add_plugin_from(&mut self, path: &str) -> () {
        let plugin_lib = unsafe { libloading::Library::new(path) };
        if let Ok(plugin_lib) = plugin_lib {
            let plugin_loader: PluginLoader = unsafe { plugin_lib.get(b"plugin") };

            match plugin_loader {
                Ok(plugin_loader) => {
                    let mut plugins = plugin_loader();
                    self.plugin_data.libraries.push(plugin_lib);

                    for plugin in &mut plugins.drain(..) {
                        self.add_plugin(plugin);
                    }
                }
                Err(err) => {
                    println!("Error loading plugin: {:?} because {}", path, err);
                }
            }
        }
    }

    fn add_plugin(&mut self, plugin: Box<dyn Plugin>) -> () {
        let mut plugin = plugin;
        self.simulation_state
            .add_particle_definition(plugin.register().into());
        self.plugin_data.plugins.push(plugin);
    }

    pub fn set_particle(&mut self, x: usize, y: usize, particle: Particle) -> () {
        self.simulation_state.set_particle_at(x, y, particle);
    }
}

pub struct Vec2i {
    pub x: isize,
    pub y: isize,
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
        let image = Image::gen_image_color(width as u16, height as u16, BLACK);
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
                name: String::from("empty"),
                color: BLACK,
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
            .get(name)
            .unwrap_or(&Particle::INVALID.id)
    }

    pub(crate) fn add_particle_definition(
        &mut self,
        particle_definition: ParticleCommonData,
    ) -> () {
        self.particle_definitions.push(particle_definition);
        self.particle_name_to_id.insert(
            self.particle_definitions.last().unwrap().name.clone(),
            self.particle_definitions.len() as u8 - 1,
        );

        // Print the name of the particle definition
        println!(
            "Added particle definition: {}",
            self.particle_definitions.last().unwrap().name
        );
    }

    pub(crate) fn get_particle_name(&self, id: usize) -> &String {
        &self.particle_definitions[id].name
    }

    pub(crate) fn get_particle_color(&self, id: usize) -> &Color {
        &self.particle_definitions[id].color
    }

    pub fn get(&self, x: i32, y: i32) -> Particle {
        let local_x = (self.current_x as i32 - x) as usize;
        let local_y = (self.current_y as i32 - y) as usize;

        if !self.is_inside_at(local_x, local_y) {
            return Particle::INVALID;
        }

        self.particles[local_y][local_x]
    }

    pub(crate) fn set_particle_at(&mut self, x: usize, y: usize, particle: Particle) -> () {
        if !self.is_inside_at(x, y) {
            return;
        }

        self.set_particle_at_unchecked(x, y, particle);
    }

    pub(crate) fn set_particle_at_unchecked(
        &mut self,
        x: usize,
        y: usize,
        particle: Particle,
    ) -> () {
        self.particles[y][x] = particle;
        self.particles[y][x].clock = !self.clock;
        let mut color = self.particle_definitions[particle.id as usize].color;
        color.a = (particle.light as f32 * TO_NORMALIZED_COLOR) * 0.15 + 0.85; // I don't like magic numbers, but for now...
                                                                               // print the light
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

    pub(crate) fn update(
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
    pub fn gen_range(&self, min: i32, max: i32) -> i32 {
        rand::gen_range(min - 1, max + 1)
    }

    pub fn is_empty(&self, x: i32, y: i32) -> bool {
        self.get(x, y) == Particle::EMPTY
    }

    pub fn random_sign(&self) -> i32 {
        rand::gen_range(0, 2) * 2 - 1
    }

    pub(crate) fn draw(&mut self) -> () {
        self.texture.update(&self.image);

        // Draw the texture
        draw_texture_ex(
            &self.texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );
    }
}
