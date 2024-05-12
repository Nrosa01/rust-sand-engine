// use egui_macroquad::macroquad::{experimental::camera::mouse, input::{is_mouse_button_down, mouse_position, mouse_wheel}, window::{screen_height, screen_width}};
use egui_macroquad::macroquad::{
    color::{hsl_to_rgb, rgb_to_hsl, Color, WHITE},
    input::*,
    shapes::draw_circle_lines,
    window::*,
};

use crate::{push_command, Command, Entity};

// I'm just mapping the mouse position to the texture coordinates
fn mouse_pos_to_square(width: usize, height: usize) -> (isize, isize) {
    let (mouse_x, mouse_y) = mouse_position();
    let min = screen_height().min(screen_width());
    let x = (mouse_x - (screen_width() - min) / 2.0) / min * width as f32;
    let y = (mouse_y - (screen_height() - min) / 2.0) / min * height as f32;

    (x as isize, y as isize)
}

pub struct Brush {
    radius: isize,
    mouse_captured: bool,
    brush_color: Color,
    mouse_hidden: bool,
}

impl Brush {
    pub fn new() -> Self {
        Brush {
            radius: 40,
            mouse_captured: false,
            brush_color: WHITE,
            mouse_hidden: false,
        }
    }
}

impl Entity for Brush {
    fn receive_command(&mut self, command: &Command) {
        match command {
            Command::NewBackgroundColor(new_color) => {
                // We are inverting the lightness of the color based on the new background color
                let new_color: Color = (*new_color).into();
                let mut hsl = rgb_to_hsl(new_color);
                hsl.2 = 1.0 - hsl.2;
                let rgb = hsl_to_rgb(hsl.0, hsl.1, hsl.2);
                self.brush_color = rgb;
            }
            Command::SetMouseHidden(hidden) => self.mouse_hidden = *hidden,
            Command::SetBrushSize(size) => self.radius = *size,
            _ => {}
        }
    }

    fn handle_input(&mut self) {
        // In wasm the browser handles the mouse wheel
        // That allows us to use an slider and many other input methods to control the brush size
        #[cfg(not(target_family = "wasm"))]
        {
            let mouse_wheel = mouse_wheel().1;
            if mouse_wheel != 0.0 {
                let sim_width = 300;

                let sensitivity = crate::WINDOW_WIDTH as isize / sim_width as isize * 5;
                let sign = mouse_wheel.signum() as isize;
                self.radius = self.radius + sign * sensitivity;

                if self.radius < 10 {
                    self.radius = 10;
                }
            }
        }

        if is_mouse_button_down(MouseButton::Left) && !self.mouse_captured {
            let radius = self.radius;
            push_command(Command::SimulationMethod(Box::new(move |simulation| {
                let sim_width = simulation.get_width();
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
        if !self.mouse_captured && !self.mouse_hidden {
            let (mouse_x, mouse_y) = mouse_position();
            draw_circle_lines(mouse_x, mouse_y, self.radius as f32, 1.0, self.brush_color);
        }
    }

    #[cfg(not(target_family = "wasm"))]
    fn ui(&mut self, egui_ctx: &egui_macroquad::egui::Context) {
        self.mouse_captured = egui_ctx.wants_pointer_input();
    }
}
