use app_core::Plugin;

#[derive(Debug)]
struct Water;

impl Plugin for Water {
    fn register(&mut self) -> app_core::ParticleDefinition {
        app_core::ParticleDefinition {
            name: String::from("Water"),
            color: 0x00FFFF,
            update_func: |state, x, y| {
                if y > 0 {
                    if state.get_particle_id(x, y - 1) == 0 {
                        state.set_particle(x, y, 0);
                        state.set_particle(x, y - 1, 2);
                    }
                }
            },
        }
    }
}

#[no_mangle]
pub fn plugin() -> Box<dyn Plugin> {
    Box::new(Water)
}
