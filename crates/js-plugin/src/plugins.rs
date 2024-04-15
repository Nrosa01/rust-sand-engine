use app_core::Particle;
use app_core::ParticleApi;
use app_core::PluginResult;
use app_core::api::Plugin;
use crate::json_converter::build_update_func;
use crate::json_converter::to_plugin_result;

pub struct JSPlugin
{
    update:Box<dyn Fn(&JSPlugin, Particle, &mut ParticleApi)>,
    plugin_data: PluginResult,
}

impl JSPlugin
{
    pub fn new(json: String) -> Result<JSPlugin, String>
    {
        let json_result = json::parse(&json);

        match json_result
        {
            Ok(json) => 
            {
                let plugin_data = to_plugin_result(&json)?;
                let update_func = build_update_func(&json)?;

              
                Ok(
                    JSPlugin
                    {
                        update: update_func,
                        plugin_data: plugin_data,
                    }
                )
            },
            Err(error) => 
            {
                return Err(format!("Error parsing JSON: {}", error).to_string());
            },
        }

    }
}

impl Plugin for JSPlugin
{
    fn update(&self, particle: Particle, api: &mut ParticleApi)
    {
        (self.update)(self, particle, api);
    }
    
    fn register(&mut self) -> app_core::PluginResult {
        PluginResult{
            name: self.plugin_data.name.clone(),
            color: self.plugin_data.color,
            extra: self.plugin_data.extra,
            alpha: self.plugin_data.alpha,
            hidden_in_ui: self.plugin_data.hidden_in_ui,
        }
    }
}