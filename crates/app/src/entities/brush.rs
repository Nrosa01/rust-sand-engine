#[cfg(not(target_family = "wasm"))]
use egui_macroquad::macroquad;
use macroquad::prelude::*;

use crate::{push_command, Command, Entity, WINDOW_WIDTH};

// I'm just mapping the mouse position to the texture coordinates
fn mouse_pos_to_square(width: usize, height: usize) -> (isize, isize) {
    let (mouse_x, mouse_y) = mouse_position();
    let min = screen_height().min(screen_width());
    let x = (mouse_x - (screen_width() - min) / 2.0) / min * width as f32;
    let y = (mouse_y - (screen_height() - min) / 2.0) / min * height as f32;

    (x as isize, y as isize)
}

pub struct Brush{
    radius: isize,
    mouse_captured: bool,
}

impl Brush{
    pub fn new() -> Self{
        Brush{
            radius: 40,
            mouse_captured: false,
        }
    }
}

impl Entity for Brush{
    fn handle_input(&mut self){
        let mouse_wheel = mouse_wheel().1;
        if mouse_wheel != 0.0 {
            let sim_width = 300;

            let sensitivity =  WINDOW_WIDTH as isize / sim_width as isize * 5;
            let sign = mouse_wheel.signum() as isize;
            self.radius = self.radius + sign * sensitivity;

            if self.radius < 10 {
                self.radius = 10;
            }
        }

        if is_mouse_button_down(MouseButton::Left) && !self.mouse_captured {
            let radius = self.radius;
            push_command(Command::SimulationMethod(Box::new(move |simulation|
            {
                let sim_width  = simulation.get_width();
                let sim_height = simulation.get_height();
    
                let (mouse_x, mouse_y) = mouse_pos_to_square(sim_width, sim_height);
                let screen_ratio_to_texture =
                    screen_height().min(screen_width()) / (sim_width.min(sim_height)) as f32;
    
                let radius = (radius as f32 / screen_ratio_to_texture) as isize;
    
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
    
                        simulation.set_selected_particle(pos_x as usize, pos_y as usize);
                    }
                }
            })));
        }
    }

    fn draw(&self) {
        if !self.mouse_captured {
            let (mouse_x, mouse_y) = mouse_position();
            draw_circle_lines(mouse_x, mouse_y, self.radius as f32, 1.0, WHITE);
        }
    }

    #[cfg(not(target_family = "wasm"))]
    fn ui(&mut self, egui_ctx: &egui_macroquad::egui::Context) {
        self.mouse_captured = egui_ctx.wants_pointer_input();
    }
}