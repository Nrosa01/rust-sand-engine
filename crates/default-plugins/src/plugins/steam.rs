use app_core::*;
use crate::*;

pub struct Steam {
    collision_targets: [u8; 1]
}

impl Steam {
    pub fn new() -> Self {
        Steam { collision_targets: [0]}
    }
}

impl Plugin for Steam {
    fn register(&mut self) -> ParticleCommonData {
        ParticleCommonData {
            name: String::from("Steam"),
            color: app_core::Color::from_rgba(128,128,128,128),
        }
    }

    fn update(&self, cell: Particle, api: &mut ParticleApi) {
        let random_horizontal = api.gen_range(-1, 1);
        let down = 1;
        
        let subtract = api.gen_range(-1, 0) as i8;
        
        // Use checked function to avoid overflow and hadle flow better
        match cell.light.checked_add_signed(subtract)
        {
            Some(result) => 
            {
                let mut cell = cell;
                cell.light = result;
                let _ = swap_if_match_using(api, random_horizontal, down, &self.collision_targets, cell) || api.set(0, 0, cell);
            },
            None => 
            {
                api.set(0, 0, Particle::EMPTY);
            }
        }
    }
}