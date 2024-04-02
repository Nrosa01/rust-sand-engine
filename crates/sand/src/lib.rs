use app_core::{Plugin, GameState};

#[derive(Debug)]
struct Sand;

impl Plugin for Sand {
    fn register(&mut self) -> (String, u32, fn(&mut GameState, usize, usize) -> ()) {
        (String::from("Sand"), 0xFFFF00, |state, x, y| {
            if y > 0 {
                if state.get_particle_id(x, y - 1) == 0 {
                    state.set_particle(x, y, 0);
                    state.set_particle(x, y - 1, 1);
                }
            }
        })
    }
}

#[no_mangle]
pub fn plugin() -> Box<dyn Plugin> {
    Box::new(Sand)
}
