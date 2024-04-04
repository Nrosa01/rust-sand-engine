pub mod sand;
pub use sand::*;
pub mod water;
pub use water::*;
pub use app_core::api::ParticleApi;

pub fn swap_if_match(api: &mut ParticleApi, x: i32, y: i32, collision_targets: &[u8]) -> bool {
    if api.is_any_particle_at(x, y, collision_targets) {
        return api.swap(x, y);
    }
    false
}

pub fn move_if_empty(api: &mut ParticleApi, x: i32, y: i32) -> bool {
    if api.is_empty(x, y) {
        return api.move_to(x, y);
    }
    false
}

