use crate::*;
use app_core::*;

pub struct Lava 
{
    rock_id: u8,
    water_id: u8,
    steam_id: u8,
}

impl Lava {
    pub fn new() -> Self {
        Lava {
            rock_id: 0,
            water_id: 0,
            steam_id: 0,
        }
    }

    pub fn try_heat_rock(&self, cell: Particle, api: &mut ParticleApi) -> bool {
        // Search for rock below, below left, and below right
        if cell.extra > 5 {
            return false;
        }

        let rock_id = self.rock_id;

        // Create tuple array to iterate over
        // I should expose Vec2 as tuples are less eficient somehow
        let directions = [(0, -1), (-1, -1), (1, -1)];

        for (x, y) in directions.iter() {
            // Check below
            let x = *x;
            let y = *y;
            if api.get(x, y).id == rock_id {
                let mut cell = cell;
                cell.extra += 1;
                api.set(0, 0, cell);

                // As we already have a mut particl3e, we can reuse it to set a new rock with a different extra value
                cell.id = rock_id;
                api.set(x, y, cell);
                return false; // Always return false to continue the chain of commands below
            }
        }

        false
    }
}

impl Plugin for Lava {
fn register(&mut self) -> PluginResult {
        PluginResult {
            name: String::from("Lava"),
            color: app_core::Color::from_rgba(255, 12, 12, 255),
            alpha: Vec2 { x: 1.0, y: 1.0 },
            ..Default::default()
        }
    }

    fn update(&self, api: &mut ParticleApi) {
        let cell = api.get_current();
        let random_horizontal = api.gen_range(-1, 1);
        let down = -1;

        let _ = try_convert(
            api,
            0,
            1,
            self.water_id,
            self.steam_id,
        ) || try_convert(
            api,
            0,
            -1,
            self.water_id,
            self.rock_id,
        )   || self.try_heat_rock(cell, api)
            || move_if_empty(api, 0, down)
            || move_if_empty(api, random_horizontal, down)
            || move_if_empty(api, -random_horizontal, down)
            || move_if_empty(api, random_horizontal, 0)
            || move_if_empty(api, -random_horizontal, 0);
    }

    fn on_plugin_changed(&mut self, api: &ParticleApi) {
        self.water_id = api.id_from_name("Water");
        self.rock_id = api.id_from_name("Rock");
        self.steam_id = api.id_from_name("Steam");
    }
}
