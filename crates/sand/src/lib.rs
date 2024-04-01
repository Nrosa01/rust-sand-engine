use app_core::Plugin;

#[derive(Debug)]
struct Counter;

impl Plugin for Counter {
    fn hook(&mut self, game_state: &mut app_core::GameState) {
        let (x, y) = (game_state.current_x, game_state.current_y);

        if y > 0 {
            if game_state.get_particle_id(x, y) == 1 && game_state.get_particle_id(x, y - 1) == 0 {
                game_state.set_particle(x, y, 0);
                game_state.set_particle(x, y - 1, 1);
            }
        }

        // Ok(())
    }
}

#[no_mangle]
pub fn plugin() -> Box<dyn Plugin> {
    Box::new(Counter)
}
