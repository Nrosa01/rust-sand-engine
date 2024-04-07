use app_core::*;
use crate::*;
use super::super::*;

pub struct Water{
    collision_targets: [u8; 2]
}

impl Water {
    pub fn new() -> Self {
        Water  { collision_targets: [0,2] }
    }

    pub fn swap_if_match(api: &mut ParticleApi, x: i32, y: i32, collision_targets: &[u8], cell: &mut Particle) -> bool {
        if api.is_any_particle_at(x, y, collision_targets) {
            cell.extra = if x == 1 { 1 } else { 2 }; // Storing the direction of the swap
            return api.swap_using(x, y, *cell);
        }
        false
    }
}

impl Plugin for Water {
    fn register(&mut self) -> PluginResult {
        PluginResult {
            name: String::from("Water"),
            color: app_core::Color::from_hex(0x00FFFF),
            alpha: Vec2 { x: 1.0, y: 1.0 },
            ..Default::default()
        }
    }

    fn update(&self, p: Particle, api: &mut ParticleApi) {
        let dir_x = if p.extra == 0  {api.gen_range(-1, 1)} else { if p.extra == 1 {1} else {-1} };
        let dir_y = -1;

        let mut p = p;
        let _ = swap_if_match(api, 0, dir_y, &self.collision_targets) || 
                swap_if_match(api, dir_x, dir_y, &self.collision_targets) || 
                swap_if_match(api, -dir_x, dir_y, &self.collision_targets) || 
                Water::swap_if_match(api, dir_x, 0, &self.collision_targets, &mut p) || 
                Water::swap_if_match(api, -dir_x, 0, &self.collision_targets, &mut p) ||
                api.set(0, 0, api.new_particle(p.id)); // If no swap was made, I create a new particle to reset the extra field
                // This is because you can't directly modify an existing particle, you have to set a new one in its place
    }

    fn post_update(&mut self, api: &ParticleApi) {
        self.collision_targets[1] = api.id_from_name("Dust");
    }
}
