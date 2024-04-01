use app_core::{GameState, Plugin};
use std::error::Error;
use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 2000;
const HEIGHT: usize = 2000;
const RADIUS: usize = 20;

fn main() -> Result<(), Box<dyn Error>> {
    let plugin_name = std::env::args()
        .nth(1)
        .expect("Provide the library name as an argument (e.g libhello_world.dylib)");

    // Be careful about explicitly calling Library::close, as it might deinitialize
    // libstd funcions such as `Box::drop`. Drop order is important. and should be
    // taken into consideration.
    let plugin_lib = unsafe { libloading::Library::new(&plugin_name) }?;
    let plugin_loader: libloading::Symbol<fn() -> Box<dyn Plugin>> =
        unsafe { plugin_lib.get(b"plugin") }?;
    let mut plugin = plugin_loader();
    
    // Create vector of Test structs
    let mut game_state = GameState::new(WIDTH, HEIGHT);

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
                            game_state.set_particle(x as usize, y as usize, 1);
                        }
                    }
                }
            }
        }

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                game_state.current_x = x;
                game_state.current_y = y;

                plugin.hook(&mut game_state);
                // if y > 0 {
                //     if game_state.get_particle_id(x, y) == 1 && game_state.get_particle_id(x, y - 1) == 0{
                //         game_state.set_particle(x, y, 0);
                //         game_state.set_particle(x, y - 1, 1);
                //     }
                // }
            }
        }

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
            println!("FPS: {}", frame_count / elapsed.as_secs());
            frame_count = 0;
            timer = std::time::Instant::now();
        }
    }

    Ok(())
}
