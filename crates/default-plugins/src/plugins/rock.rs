use crate::*;
use app_core::*;

pub struct Rock {
    water_id: u8,
}

impl Rock {
    pub fn new() -> Self {
        Rock { water_id: 0 }
    }
}

impl Plugin for Rock {
fn register(&mut self) -> PluginResult {
        PluginResult {
            name: String::from("Rock"),
            color: app_core::Color::from_rgba(123, 133, 145, 255),
            ..Default::default()
        }
    }

    fn update(&self, api: &mut ParticleApi) {
        let cell = api.get_current();

        if cell.extra == 0 {
            return;
        }

        let mut cell = cell;

        let water_id = self.water_id;

        for neighbor in ParticleApi::NEIGHBORS {
            // For each neighbour, is if type water, turn it into rock and set the extra to -1
            // We do the same if the neighbour is the same type as the cell, thay way we can transfer the extra to the neighbour
            let neighor_id = api.get(neighbor.x, neighbor.y).id;
            if neighor_id== water_id || neighor_id == cell.id {
                match cell.extra.checked_add_signed(-1) {
                    Some(result) => {
                        cell.extra = result;
                    }
                    None => {
                        cell.extra = 0;
                    }
                }
                api.set(neighbor.x, neighbor.y, cell);
                api.set(0, 0, cell);
            }
            

            if cell.extra == 0 {
                return;
            }
        }
    }

    fn on_plugin_changed(&mut self, api: &ParticleApi) {
        self.water_id = api.id_from_name("Water");
    }
}
