pub mod sand;
pub use sand::*;
pub mod water;
pub use water::*;
pub mod dust;
pub use dust::*;
pub mod steam;
pub use steam::*;
pub mod rock;
pub use rock::*;
pub mod lava;
pub use lava::*;

use app_core::Particle;
pub use app_core::api::ParticleApi;

pub fn swap_if_match(api: &mut ParticleApi, x: i32, y: i32, collision_targets: &[u8]) -> bool {
    if api.is_any_particle_at(x, y, collision_targets) {
        return api.swap(x, y);
    }
    false
}

pub fn swap_if_match_using(api: &mut ParticleApi, x: i32, y: i32, collision_targets: &[u8], cell: Particle) -> bool {
    if api.is_any_particle_at(x, y, collision_targets) {
        return api.swap_using(x, y, cell);
    }
    false
}

pub fn move_if_empty(api: &mut ParticleApi, x: i32, y: i32) -> bool {
    if api.is_empty(x, y) {
        return api.move_to(x, y);
    }
    false
}

pub fn try_convert(api: &mut ParticleApi, x: i32, y: i32, target: u8, to: u8) -> bool {
    if api.get(x, y) == target {
        return api.set(x, y, api.new_particle(to));
    }
    false
}