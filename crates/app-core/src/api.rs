use std::vec;

use macroquad::prelude::*;
use rustc_hash::FxHashMap;

use crate::{Plugin, PluginResult};

pub struct Empty;

impl Plugin for Empty {
    fn register(&mut self) -> PluginResult {
        PluginResult {
            name: String::from("Empty"),
            color: 0x000000,
        }
    }

    fn update(&self, _cell: Particle, _api: &mut SimulationState) {}
}

#[derive(Clone, Debug, Copy)]
pub struct Particle {
    pub id: usize,
    pub clock: bool,
}

impl Particle {
    pub const EMPTY: Particle = Particle {
        id: 0,
        clock: false,
    };

    pub(crate) const INVALID: Particle = Particle {
        id: usize::MAX,
        clock: false,
    };
}

impl PartialEq for Particle {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl PartialEq<usize> for Particle {
    fn eq(&self, other: &usize) -> bool {
        self.id == *other
    }
}

impl From<usize> for Particle {
    fn from(id: usize) -> Self {
        Particle { id, clock: false }
    }
}

#[derive(Debug)]
pub struct ParticleCommonData {
    pub name: String,
    pub color: Color,
}

pub struct PluginData {
    pub(crate) libraries: Vec<libloading::Library>,
    pub(crate) plugins: Vec<Box<dyn Plugin>>,
}

impl PluginData {
    pub fn new() -> PluginData {
        PluginData {
            libraries: Vec::new(),
            plugins: vec![Box::new(Empty)],
        }
    }
}

pub struct Simulation {
    simulation_state: SimulationState,
    plugin_data: PluginData,
}

impl Simulation {
    pub fn new(width: usize, height: usize) -> Simulation {
        Simulation {
            simulation_state: SimulationState::new(width, height),
            plugin_data: PluginData::new(),
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
        self.simulation_state.update(&mut self.plugin_data.plugins);
    }

    pub fn draw(&mut self) -> () {
        self.simulation_state.draw();
    }

    pub fn get_particle_name(&self, id: usize) -> &String {
        &self.simulation_state.particle_definitions[id].name
    }

    pub fn get_particle_color(&self, id: usize) -> &Color {
        &self.simulation_state.particle_definitions[id].color
    }

    pub fn add_plugin_from(&mut self, path: &str) -> () {
        let plugin_lib = unsafe { libloading::Library::new(path) };
        if let Ok(plugin_lib) = plugin_lib {
            let plugin_loader: Result<
                libloading::Symbol<fn() -> Box<dyn Plugin>>,
                libloading::Error,
            > = unsafe { plugin_lib.get(b"plugin") };
            if let Ok(plugin_loader) = plugin_loader {
                let mut plugin = plugin_loader();
                self.simulation_state
                    .add_particle_definition(plugin.register().into());
                self.plugin_data.plugins.push(plugin);
                self.plugin_data.libraries.push(plugin_lib);
            }
        }
    }

    pub fn set_particle(&mut self, x: usize, y: usize, particle: Particle) -> () {
        self.simulation_state.set_particle_at(x, y, particle);
    }
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
    particle_name_to_id: FxHashMap<String, usize>,
}

impl SimulationState {
    pub fn new(width: usize, height: usize) -> SimulationState {
        let image = Image::gen_image_color(width as u16, height as u16, BLACK);
        let texture = Texture2D::from_image(&image);
        texture.set_filter(FilterMode::Nearest); // Set the filter mode to nearest to avoid blurring the pixels

        SimulationState {
            particles: vec![
                vec![
                    Particle {
                        id: 0,
                        clock: false
                    };
                    width
                ];
                height
            ],
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

    pub fn id_from_name(&self, name: &str) -> usize {
        *self.particle_name_to_id.get(name).unwrap()
    }

    pub(crate) fn add_particle_definition(&mut self, particle_definition: ParticleCommonData) -> () {
        self.particle_definitions.push(particle_definition);
        self.particle_name_to_id.insert(
            self.particle_definitions.last().unwrap().name.clone(),
            self.particle_definitions.len() - 1,
        );

        // Print the name of the particle definition
        println!(
            "Added particle definition: {}",
            self.particle_definitions.last().unwrap().name
        );
    }

    pub(crate) fn set_particle_at(&mut self, x: usize, y: usize, particle: Particle) -> () {
        if !self.is_inside(x, y) {
            return;
        }

        self.particles[y][x] = particle;
        self.particles[y][x].clock = !self.clock;
        self.image
            .set_pixel(x as u32, y as u32, self.particle_definitions[particle.id].color);
    }

    pub fn get(&self, x: i32, y: i32) -> Particle {
        let local_x = (self.current_x as i32 - x) as usize;
        let local_y = (self.current_y as i32 - y) as usize;

        if !self.is_inside(local_x, local_y) {
            return Particle::INVALID; // TODO: Change this to return a particle with id max usize value
        }

        self.particles[local_y][local_x]
    }

    pub fn set(&mut self, x: i32, y: i32, particle: Particle) -> () {
        let local_x = (self.current_x as i32 - x) as usize;
        let local_y = (self.current_y as i32 - y) as usize;

        if !self.is_inside(local_x, local_y) {
            return;
        }

        self.particles[local_y][local_x] = particle;
        self.particles[local_y][local_x].clock = !self.clock;
        self.image.set_pixel(
            local_x as u32,
            local_y as u32,
            self.particle_definitions[particle.id].color,
        );
    }

    pub fn is_inside(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    pub(crate) fn update(&mut self, plugins: &mut Vec<Box<dyn Plugin>>) -> () {
        for y in 0..self.height {
            for x in 0..self.width {
                self.current_x = x;
                self.current_y = y;
                let current_particle = &self.particles[y][x];
                let particle_id = self.particles[y][x]; // Not using getter here to avoid the if check that will never be true here
                if particle_id == 0 || current_particle.clock != self.clock {
                    continue;
                }

                let plugin = &mut plugins[particle_id.id];
                plugin.update(self.particles[y][x], self);
            }
        }

        self.clock = !self.clock;

        // Post update
        for plugin in plugins.iter_mut() {
            plugin.post_update(self);
        }
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
