use crate::api::*;

pub struct Simulation {
    simulation_state: SimulationState,
    plugin_data: PluginData,
    order_scheme: OrderSchemes,
    selected_plugin: u8,
}

impl Simulation {
    pub fn new(width: usize, height: usize) -> Simulation {
        Simulation {
            simulation_state: SimulationState::new(width, height),
            plugin_data: PluginData::new(),
            order_scheme: OrderSchemes::new(width, height),
            selected_plugin: 1,
        }
    }

    pub fn get_selected_plugin(&self) -> u8 {
        self.selected_plugin
    }

    pub fn select_next_plugin(&mut self)
    {
        self.selected_plugin = (self.selected_plugin + 1) % self.get_plugin_count() as u8;
    }

    pub fn select_previous_plugin(&mut self)
    {
        self.selected_plugin = (self.selected_plugin + self.get_plugin_count() as u8 - 1) % self.get_plugin_count() as u8;
    }

    pub fn set_selected_plugin(&mut self, selected_plugin: u8) -> () {
        self.selected_plugin = selected_plugin;
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

    pub fn get_buffer(&self) -> &[u8] {
        self.simulation_state.get_buffer()
    }

    pub fn get_particles(&self) -> &Vec<Vec<Particle>> {
        self.simulation_state.get_particles()
    }

    pub fn get_particle_name(&self, id: usize) -> Result<&String, String> {
        if id >= self.get_plugin_count() {
            return Err("Particle with id ".to_string() + &id.to_string() + " not found");
        }

        Ok(&self.simulation_state.get_particle_name(id))
    }

    pub fn get_particle_color(&self, id: usize) -> Result<&[u8; 4], String> {
        if id >= self.get_plugin_count() {
            return Err("Particle with id ".to_string() + &id.to_string() + " not found");
        }

        Ok(&self.simulation_state.get_particle_color(id))
    }

    pub fn add_plugin(&mut self, plugin: Box<dyn Plugin>) -> () {
        let mut plugin = plugin;

        // The simulation state returns the id of the particle definition if it already exists
        let id = self.simulation_state.add_or_replace_particle_definition(plugin.register().into());

        match id
        {
            Some(id) => {
                self.plugin_data.plugins[id] = plugin;
                // Maybe it was just a color change
                self.simulation_state.repaint();
            },
            None => {
                self.plugin_data.plugins.push(plugin);
            }
        }
        
        self.plugin_data.notify(&self.simulation_state);
    }

    pub fn remove_plugin(&mut self, id: u8) -> () {
        self.simulation_state.remove_particle_definition(id);
        self.plugin_data.plugins.remove(id as usize);
        self.repaint();
        self.plugin_data.notify(&self.simulation_state);

        self.selected_plugin = self.selected_plugin.min(self.get_plugin_count() as u8 - 1);
    }

    pub fn add_plugins(&mut self, plugins: Vec<Box<dyn Plugin>>) -> () {
        for plugin in plugins {
            self.add_plugin(plugin);
        }
    }

    pub fn clear(&mut self) -> () {
        self.simulation_state.clear();
    }

    pub fn repaint(&mut self) -> () {
        self.simulation_state.repaint();
    }

    pub fn resize(&mut self, size: u32) -> () {
        self.simulation_state.resize(size);
        self.order_scheme = OrderSchemes::new(self.get_width(), self.get_height());
    }

    pub fn set_selected_particle(&mut self, x: usize, y: usize) -> () {
        self.simulation_state
            .set_particle_at_by_id(x, y, self.selected_plugin.into());
    }

    pub fn set_particle(&mut self, x: usize, y: usize, particle: Particle) -> () {
        self.simulation_state
            .set_particle_at_by_id(x, y, particle.id);
    }
}
