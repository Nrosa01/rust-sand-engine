use egui_macroquad::{egui, macroquad::texture::Texture2D};
use js_plugin::plugins::JSPlugin;

use crate::*;

const SIMULATION_STARTING_WIDTH: usize = 300;
const SIMULATION_STARTING_HEIGHT: usize = 300;

pub struct Universe {
    simulation: Simulation,
    texture: Texture2D,
    paused: bool,
    #[cfg(not(target_family = "wasm"))]
    native_plugin_loader: DylibLoader,
}

impl Universe {
    pub fn new() -> Self {
        let simulation = Simulation::new(SIMULATION_STARTING_WIDTH, SIMULATION_STARTING_HEIGHT);
        let texture = Texture2D::from_rgba8(
            SIMULATION_STARTING_WIDTH as u16,
            SIMULATION_STARTING_HEIGHT as u16,
            &simulation.get_buffer(),
        );
        Universe {
            simulation: simulation,
            texture: texture,
            paused: false,
            #[cfg(not(target_family = "wasm"))]
            native_plugin_loader: DylibLoader::new(),
        }
    }

    pub fn resize(&mut self, size: u32) {
        self.simulation.resize(size);
        resize_texture(
            &mut self.texture,
            size as u32,
            size as u32,
            self.simulation.get_buffer(),
        );
    }

    fn select_particle(&mut self, id: usize) {
        self.simulation.set_selected_plugin(id as u8);
    }

    pub fn set_paused(&mut self, paused: bool) {
        self.paused = paused;
    }
}

impl Entity for Universe {
    fn init(&mut self) {
        self.texture.set_filter(FilterMode::Nearest);

        #[cfg(not(target_family = "wasm"))]
        {
            let plugin_path = std::env::current_exe()
                .unwrap()
                .parent()
                .unwrap()
                .join(format!("default_plugins.{}", DylibLoader::extension()));
            let plugins = self
                .native_plugin_loader
                .load(plugin_path.to_str().unwrap())
                .unwrap();
            self.simulation.add_plugins(plugins);
        }
    }

    fn receive_command(&mut self, command: &Command) {
        match command {
            Command::NewPlugin(json) => {
                let plugin = JSPlugin::new(json);
                match plugin {
                    Ok(plugin) => {
                        self.simulation.add_plugin(Box::new(plugin));
                    }
                    Err(error) => {
                        println!("Error loading plugin: {}", error);
                    }
                }
            }
            Command::CanvasSize(size) => {
                self.resize(*size);
            }
            Command::Clear => self.simulation.clear(),
            Command::Pause(is_paused) => {
                self.set_paused(*is_paused);
            }
            Command::SimulationMethod(method) => method(&mut self.simulation),
            Command::ParticleSelected(id) => self.select_particle(*id as usize),
            _ => {}
        }
    }

    fn update(&mut self) {
        if !self.paused {
            self.simulation.update();
        }
    }

    #[cfg(not(target_family = "wasm"))]
    fn handle_input(&mut self) {
        if is_key_pressed(KeyCode::Right)
            || is_key_pressed(KeyCode::D)
            || is_key_pressed(KeyCode::S)
        {
            self.simulation.select_next_plugin();
        }

        if is_key_pressed(KeyCode::Left) || is_key_pressed(KeyCode::A) || is_key_pressed(KeyCode::W) {
            self.simulation.select_previous_plugin();
        }
    }

    fn draw(&self) {
        draw_simulation(&self.texture, &self.simulation.get_buffer());
    }

    #[cfg(not(target_family = "wasm"))]
    fn ui(&mut self, egui_ctx: &egui_macroquad::egui::Context) {
        egui::Area::new(egui::Id::new("my_area"))
            .default_pos(egui::pos2(32.0, 32.0))
            .movable(true)
            .show(egui_ctx, |ui| {
                ui.label(format!("FPS: {}", get_fps()));
                for i in 0..self.simulation.get_plugin_count() {
                    let plugin = &self.simulation.get_particle_definitions()[i];

                    let should_hightlight = i == self.simulation.get_selected_plugin() as usize;
                    let name = &plugin.name;
                    let button = ui.button(name);
                    if should_hightlight {
                        button.highlight();
                    } else if button.clicked() {
                        self.select_particle(i);
                    }
                }
            });
    }
}

fn resize_texture(texture: &mut Texture2D, width: u32, height: u32, buffer: &[u8]) {
    let ctx = unsafe { get_internal_gl().quad_context };
    texture.texture.resize(ctx, width, height, Some(buffer));
}

fn draw_simulation(texture: &Texture2D, bytes: &[u8]) {
    let raw = texture.raw_miniquad_texture_handle();
    let ctx = unsafe { get_internal_gl().quad_context };
    raw.update(ctx, bytes);

    let pos_x = (screen_width() / 2.0 - screen_height() / 2.0).max(0.);
    let pos_y = (screen_height() / 2.0 - screen_width() / 2.0).max(0.);

    let dest_size = screen_height().min(screen_width());

    // Draw rect with transparent color
    draw_rectangle(
        pos_x,
        pos_y,
        dest_size,
        dest_size,
        Color::from_hex(0x12212b),
    );

    // Draw the texture
    draw_texture_ex(
        *texture,
        pos_x,
        pos_y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(dest_size, dest_size)),
            ..Default::default()
        },
    );
}
