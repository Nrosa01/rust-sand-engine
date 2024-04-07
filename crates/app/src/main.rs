// #![windows_subsystem = "windows"]

pub mod app_core;
pub mod integrated_particles;
use crate::app_core::api::simulation::Simulation;
use egui_macroquad::{
    egui::{self},
    macroquad,
};
use integrated_particles::plugin;
use macroquad::prelude::*;
use std::error::Error;

const WINDOW_WIDTH: i32 = 1200;
const WINDOW_HEIGHT: i32 = 600;

fn conf() -> Conf {
    Conf {
        window_title: String::from("Pixel Flow"),
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        window_resizable: false,
        ..Default::default()
    }
}

const WIDTH: usize = 300;
const HEIGHT: usize = 300;
const SENSITIVITY: isize = WINDOW_WIDTH as isize / WIDTH as isize * 5;

// No sé compo explicar esto, imagina que tenemos una pantalla de 800*400
// Esta función devuelve posiciones x e y entre 0 y 400 en el centro de la pantalla, las que se salen son negativas o mayores de 400
// Si la pantalla es de 800*800, las posiciones x e y estarán entre 0 y 800 como siempre
fn mouse_pos_to_square() -> (isize, isize) {
    let (mouse_x, mouse_y) = mouse_position();
    let min = screen_height().min(screen_width());
    let x = (mouse_x - (screen_width() - min) / 2.0) / min * WIDTH as f32;
    let y = (mouse_y - (screen_height() - min) / 2.0) / min * HEIGHT as f32;

    (x as isize, y as isize)
}

#[macroquad::main(conf)]
async fn main() -> Result<(), Box<dyn Error>> {
    macroquad::rand::srand(macroquad::miniquad::date::now() as u64);
    let mut radius: isize = 40;
    let mut hide_ui = false;

    let mut simulation = Simulation::new(WIDTH, HEIGHT);

    let mut selected_plugin = 1;

    let screen_ratio_to_texture = screen_width() / WIDTH as f32;

    let plugins = plugin();

    for plugin in plugins {
        simulation.add_plugin(plugin);
    }

    let mut capture_mouse = false;

    loop {
        if is_key_pressed(KeyCode::Right)
            || is_key_pressed(KeyCode::D)
            || is_key_pressed(KeyCode::S)
        {
            loop {
                if selected_plugin == simulation.get_plugin_count() - 1 {
                    selected_plugin = 0;
                } else {
                    selected_plugin += 1;
                }

                if !simulation
                    .get_particle_hide_in_ui(selected_plugin)
                    .unwrap_or_default()
                {
                    break;
                }
            }
        }

        if is_key_pressed(KeyCode::H) {
            hide_ui = !hide_ui;
        }

        if is_key_pressed(KeyCode::Left) || is_key_pressed(KeyCode::A) || is_key_pressed(KeyCode::W)
        {
            loop {
                if selected_plugin == 0 {
                    selected_plugin = simulation.get_plugin_count() - 1;
                } else {
                    selected_plugin -= 1;
                }

                if !simulation
                    .get_particle_hide_in_ui(selected_plugin)
                    .unwrap_or_default()
                {
                    break;
                }
            }
        }

        // Use mouse wheel to change radius
        let mouse_wheel = mouse_wheel().1;
        // Draw both mouse wheelv alues
        if mouse_wheel != 0.0 {
            let sign = mouse_wheel.signum() as isize;
            radius = radius + sign * SENSITIVITY;
        }

        // Break the loop if the user closes the window OR presses the escape key
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        if is_mouse_button_down(MouseButton::Left) && !capture_mouse {
            let (mouse_x, mouse_y) = mouse_pos_to_square();

            let radius = (radius as f32 / screen_ratio_to_texture) as isize * 2;

            for x in -radius..radius {
                for y in -radius..radius {
                    let pos_x = mouse_x + x;
                    let pos_y = mouse_y + y;

                    if pos_x < 0 || pos_y < 0 {
                        continue;
                    }

                    let distance = (x * x + y * y) as f32;

                    if distance > radius as f32 * radius as f32 {
                        continue;
                    }

                    simulation.set_particle(pos_x as usize, pos_y as usize, selected_plugin.into());
                }
            }
        }

        simulation.update();

        // Clear the screen
        clear_background(BLACK);

        egui_macroquad::ui(|egui_ctx| {
            if hide_ui {
                return;
            }

            egui::Area::new(egui::Id::new("my_area"))
                .default_pos(egui::pos2(32.0, 32.0))
                .movable(true)
                .show(egui_ctx, |ui| {
                    ui.label(format!("FPS: {}", get_fps()));
                    for i in 0..simulation.get_plugin_count() {
                        let plugin = &simulation.get_particle_definitions()[i];
                        if plugin.hide_in_ui {
                            continue;
                        }

                        let should_hightlight = i == selected_plugin;
                        let name = &plugin.name;
                        let button = ui.button(name);
                        if should_hightlight {
                            button.highlight();
                        } else if button.clicked() {
                            selected_plugin = i;
                        }
                    }
                });

            capture_mouse = egui_ctx.wants_pointer_input();
        });

        simulation.draw();

        // Draw circle line with radius at mouse position
        if !capture_mouse {
            let (mouse_x, mouse_y) = mouse_position();
            draw_circle_lines(mouse_x, mouse_y, radius as f32, 1.0, WHITE);
        }

        egui_macroquad::draw();
        next_frame().await;
    }

    Ok(())
}
