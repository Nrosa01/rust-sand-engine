use crate::api::*;

pub struct Simulation {
    simulation_state: SimulationState,
    plugin_data: PluginData,
    order_scheme: OrderSchemes,
}

type PluginLoader<'a> =
    Result<libloading::Symbol<'a, fn() -> Vec<Box<dyn Plugin>>>, libloading::Error>;

impl Simulation {
    pub fn new(width: usize, height: usize) -> Simulation {
        Simulation {
            simulation_state: SimulationState::new(width, height),
            plugin_data: PluginData::new(),
            order_scheme: OrderSchemes::new(width, height),
        }
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

    pub fn get_particle_color(&self, id: usize) -> Result<&Color, String> {
        if id >= self.get_plugin_count() {
            return Err(format!("Particle with id {} not found", id));
        }

        Ok(&self.simulation_state.get_particle_color(id))
    }

    pub fn add_plugin_from(&mut self, path: &str) -> () {
        let plugin_lib = unsafe { libloading::Library::new(path) };
        if let Ok(plugin_lib) = plugin_lib {
            let plugin_loader: PluginLoader = unsafe { plugin_lib.get(b"plugin") };

            match plugin_loader {
                Ok(plugin_loader) => {
                    let mut plugins = plugin_loader();
                    self.plugin_data.libraries.push(plugin_lib);

                    for plugin in &mut plugins.drain(..) {
                        self.add_plugin(plugin);
                    }
                }
                Err(_) => {}
            }
        }
    }

    fn add_plugin(&mut self, plugin: Box<dyn Plugin>) -> () {
        let mut plugin = plugin;
        self.simulation_state
            .add_particle_definition(plugin.register().into());
        self.plugin_data.plugins.push(plugin);
    }

    pub fn set_particle(&mut self, x: usize, y: usize, particle: Particle) -> () {
        self.simulation_state.set_particle_at(x, y, particle);
    }
}