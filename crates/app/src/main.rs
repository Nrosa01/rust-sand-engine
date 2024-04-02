use app_core::{GameState, Plugin};
use minifb::{Key, Window, WindowOptions};
use std::error::Error;

const WIDTH: usize = 400;
const HEIGHT: usize = 400;
const RADIUS: usize = 20;

fn main() -> Result<(), Box<dyn Error>> {
    let mut game_state = GameState::new(WIDTH, HEIGHT);
    let mut plugins = Vec::new(); // I need to keep the libraries open, so they won't be unloaded when going out of scope in the loop below

    let mut selected_plugin = 0;

    // Iterate all dll files in the current directory, load them and find the plugin function, if it's not exist, ignore it and close the library
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

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let mut timer = std::time::Instant::now();
    let mut frame_count = 0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // for i in buffer.iter_mut() {
        //     *i = 0; // write something more funny here!
        // }

        // Paint yellow when left mouse button is down
        if window.get_mouse_down(minifb::MouseButton::Left) {
            let (mouse_x, mouse_y) = window.get_mouse_pos(minifb::MouseMode::Clamp).unwrap();
            for y in (mouse_y - RADIUS as f32) as i32..(mouse_y + RADIUS as f32) as i32 {
                for x in (mouse_x - RADIUS as f32) as i32..(mouse_x + RADIUS as f32) as i32 {
                    if (x as f32 - mouse_x).powi(2) + (y as f32 - mouse_y).powi(2)
                        < RADIUS as f32 * RADIUS as f32
                    {
                        if x >= 0 && x < WIDTH as i32 && y >= 0 && y < HEIGHT as i32 {
                            //buffer[(x + y * WIDTH as i32) as usize] = 0xFFFF00;
                            game_state.set_particle(x as usize, y as usize, selected_plugin);
                        }
                    }
                }
            }
        }

        // Uswe a match instead of this
        let keys = window.get_keys_pressed(minifb::KeyRepeat::No);
        if keys.len() > 0 {
            match keys[0] {
                Key::Key1 => selected_plugin = 1,
                Key::Key2 => selected_plugin = 2,
                Key::Key3 => selected_plugin = 3,
                Key::Key4 => selected_plugin = 4,
                Key::Key5 => selected_plugin = 5,
                Key::Key6 => selected_plugin = 6,
                Key::Key7 => selected_plugin = 7,
                Key::Key8 => selected_plugin = 8,
                Key::Key9 => selected_plugin = 9,
                Key::Key0 => selected_plugin = 0,
                _ => {}
            }
        }


        game_state.update();
        // Draw particles, if id is 1, draw it as yellow
        game_state.draw();

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(game_state.get_buffer(), WIDTH, HEIGHT)
            .unwrap();

        // Update timer, every one second, print the fps
        let elapsed = timer.elapsed();
        frame_count += 1;
        if elapsed.as_secs() > 0 {
            println!("FPS: {} with {} plugin", frame_count / elapsed.as_secs(), selected_plugin);
            frame_count = 0;
            timer = std::time::Instant::now();
        }
    }

    Ok(())
}
