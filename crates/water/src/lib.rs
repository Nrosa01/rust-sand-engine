use app_core::{GameState, Plugin};

#[derive(Debug)]
struct Water;

impl Plugin for Water {
    fn register(&mut self) -> (String, u32, fn(&mut GameState, usize, usize) -> ()) {
        (String::from("Water"), 0x00FFFF, |state, x, y| {
            if y > 0 {
                if state.get_particle_id(x, y - 1) == 0 {
                    state.set_particle(x, y, 0);
                    state.set_particle(x, y - 1, 2);
                }
            }
        })
    }
}

#[no_mangle]
pub fn plugin() -> Box<dyn Plugin> {
    Box::new(Water)
}
