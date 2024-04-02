use app_core::{GameState, Plugin};
use macroquad::prelude::*;
use std::error::Error;
use std::thread::sleep;
use std::time::{Duration, Instant};

fn conf() -> Conf {
    Conf {
        window_title: String::from("Pixel Flow"),
        window_width: 800,
        window_height: 800,
        ..Default::default()
    }
}

const TARGET_FPS: f64 = 60.0;
const WIDTH: usize = 400;
const HEIGHT: usize = 400;

#[macroquad::main(conf)]
async fn main() -> Result<(), Box<dyn Error>> {
    let frame_time: Duration = Duration::from_secs_f64(1.0 / (TARGET_FPS + 1.0));
    let mut radius: usize = 20;

    let mut game_state = GameState::new(WIDTH, HEIGHT);
    let mut plugins = Vec::new(); // I need to keep the libraries open, so they won't be unloaded when going out of scope in the loop below

    let mut selected_plugin = 1;
    let mut image = Image::gen_image_color(WIDTH as u16, HEIGHT as u16, BLACK);
    let texture = Texture2D::from_image(&image);
    texture.set_filter(FilterMode::Nearest); // Set the filter mode to nearest to avoid blurring the pixels

    // I just search for plugins in the same directory as the executable and load them if they are valid
    for entry in std::fs::read_dir(std::env::current_exe()?.parent().unwrap())? {
        let entry = entry?;
        let path = entry.path();
        // Print the path
        if path.is_file() {
            let file_name = path.file_name().unwrap().to_str().unwrap();
            if file_name.ends_with(".dll") {
                let plugin_lib = unsafe { libloading::Library::new(&file_name) };
                if let Ok(plugin_lib) = plugin_lib {
                    let plugin_loader: Result<
                        libloading::Symbol<fn() -> Box<dyn Plugin>>,
                        libloading::Error,
                    > = unsafe { plugin_lib.get(b"plugin") };
                    if let Ok(plugin_loader) = plugin_loader {
                        let mut plugin = plugin_loader();
                        game_state.add_particle_definition(plugin.register());
                        print!("Loaded plugin: {}", file_name);
                        plugins.push(plugin_lib);
                    }
                }
            }
        }
    }

    loop {
        let frame_start = Instant::now();

        if is_key_pressed(KeyCode::Right) {
            selected_plugin += 1;
            if selected_plugin >= game_state.get_particle_definitions().len() {
                selected_plugin = 0;
            }
        }
        if is_key_pressed(KeyCode::Left) {
            if selected_plugin == 0 {
                selected_plugin = game_state.get_particle_definitions().len() - 1;
            } else {
                selected_plugin -= 1;
            }
        }

        // Use mouse wheel to change radius
        let mouse_wheel = mouse_wheel().1;
        // Draw both mouse wheelv alues
        if mouse_wheel != 0.0 {
            let new_radius = radius as i32 + mouse_wheel as i32;
            // ADD radius sign to the new_radius, so if new_radius is negative, it will substract -1, otherwise it will add 1
            let sign = new_radius.signum();
            radius = (radius as i32 + sign) as usize;
        }

        // Break the loop if the user closes the window OR presses the escape key
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        if is_mouse_button_down(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();

            // Calcula el factor de escala para convertir las coordenadas del mouse a las coordenadas de la textura
            let scale_x = image.width as f32 / screen_width();
            let scale_y = image.height as f32 / screen_height();

            // Aplica el factor de escala a las coordenadas del mouse
            let scaled_mouse_x = (mouse_x * scale_x).floor();
            let scaled_mouse_y = (mouse_y * scale_y).floor();

            let radius = radius as i32 / 2;

            for x in -radius..radius {
                for y in -radius..radius {
                    let pos_x = scaled_mouse_x + x as f32;
                    let pos_y = scaled_mouse_y + y as f32;

                    let distance_squared =
                        (pos_x - scaled_mouse_x).powi(2) + (pos_y - scaled_mouse_y).powi(2);
                    if distance_squared <= radius.pow(2) as f32 {
                        game_state.set_particle(
                            pos_x as usize,
                            pos_y as usize,
                            selected_plugin as u32,
                        );
                    }
                }
            }
        }

        game_state.update();

        // Clear the screen
        clear_background(BLACK);

        // Draw the particles by modifying the buffer
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let particle = &game_state.particles[y][x];
                let particle_definition =
                    &game_state.get_particle_definitions()[particle.id as usize];
                let color = particle_definition.color;
                image.set_pixel(x as u32, y as u32, Color::from_hex(color));
            }
        }

        texture.update(&image);

        // Draw the texture
        draw_texture_ex(
            &texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );

        // Draw the selected particle
        draw_text(
            &format!(
                "Selected particle: {}",
                game_state.get_particle_definitions()[selected_plugin].name
            ),
            10.0,
            screen_height() - 30.0,
            20.0,
            WHITE,
        );

        draw_text(&format!("FPS: {}", get_fps()), 10.0, 30.0, 30.0, RED);

        // Draw circle line with radius at mouse position
        let (mouse_x, mouse_y) = mouse_position();
        draw_circle_lines(mouse_x, mouse_y, radius as f32, 1.0, WHITE);

        next_frame().await;

        let elapsed = frame_start.elapsed();
        if elapsed < frame_time {
            sleep(frame_time - elapsed);
        }
    }
    //game_state.draw();

    Ok(())
}
