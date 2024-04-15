use app_core::*;

pub struct Steam {
    rock_id: u8,
}

impl Steam {
    pub fn new() -> Self {
        Steam { rock_id: 0 }
    }
}

impl Plugin for Steam {
    fn register(&mut self, api: &ParticleApi) -> PluginResult {
        self.on_plugin_changed(api);
        PluginResult {
            name: String::from("Steam"),
            color: app_core::Color::from_rgba(128,128,128,128),
            ..Default::default()
        }
    }

    fn update(&self, cell: Particle, api: &mut ParticleApi) {
        let random_horizontal = api.gen_range(-1, 1);
        let up = 1;
        
        let subtract = api.gen_range(-1, 0) as i8;
        
        // Use checked function to avoid overflow and hadle flow better
        match cell.light.checked_add_signed(subtract)
        {
            Some(result) => 
            {
                let mut cell = cell;
                cell.light = result;
                let _  = (api.get(random_horizontal, up) != cell && api.get(random_horizontal, up) != self.rock_id) && 
                          api.swap_using(random_horizontal, up, cell) ||
                          api.set(0, 0, cell);
            },
            None => 
            {
                api.set(0, 0, Particle::EMPTY);
            }
        }
    }

    fn on_plugin_changed(&mut self, api: &ParticleApi) {
        self.rock_id = api.id_from_name("Rock");
    }
}