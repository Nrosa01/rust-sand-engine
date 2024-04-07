use super::super::api::*;

pub struct Simulation {
    simulation_state: SimulationState,
    plugin_data: PluginData,
    order_scheme: OrderSchemes,
}

// type PluginLoader<'a> =
//     Result<libloading::Symbol<'a, fn() -> Vec<Box<dyn Plugin>>>, libloading::Error>;

impl Simulation {
    pub fn new(width: usize, height: usize) -> Simulation {
        Simulation {
            simulation_state: SimulationState::new(width, height),
            plugin_data: PluginData::new(),
            order_scheme: OrderSchemes::new(width, height),
        }
    }

    pub fn get_particle_definitions(&self) -> &Vec<ParticleCommonData> {
        &self.simulation_state.get_particle_definitions()
    }

    pub fn get_plugin_count(&self) -> usize {
        self.plugin_data.plugins.len()
    }

    pub fn get_width(&self) -> usize {
        self.simulation_state.width()
    }

    pub fn get_height(&self) -> usize {
        self.simulation_state.height()
    }

    pub fn update(&mut self) -> () {
        self.simulation_state.update(
            &mut self.plugin_data.plugins,
            &self.order_scheme.get_ciclying(),
        );
    }

    pub fn draw(&mut self) -> () {
        self.simulation_state.draw();
    }

    pub fn get_particle_name(&self, id: usize) -> Result<&String, String> {
        if id >= self.get_plugin_count() {
            return Err(format!("Particle with id {} not found", id));
        }

        Ok(&self.simulation_state.get_particle_name(id))
    }

    pub fn get_particle_hide_in_ui(&self, id: usize) -> Result<bool, String> {
        if id >= self.get_plugin_count() {
            return Err(format!("Particle with id {} not found", id));
        }

        Ok(self.simulation_state.get_particle_definitions()[id].hide_in_ui)
    }

    pub fn get_particle_color(&self, id: usize) -> Result<&Color, String> {
        if id >= self.get_plugin_count() {
            return Err(format!("Particle with id {} not found", id));
        }

        Ok(&self.simulation_state.get_particle_color(id))
    }

    pub fn add_plugin(&mut self, plugin: Box<dyn Plugin>) -> () {
        let mut plugin = plugin;
        self.simulation_state
            .add_particle_definition(plugin.register().into());
        self.plugin_data.plugins.push(plugin);
    }

    pub fn set_particle(&mut self, x: usize, y: usize, particle: Particle) -> () {
        self.simulation_state.set_particle_at_by_id(x, y, particle.id);
    }
}