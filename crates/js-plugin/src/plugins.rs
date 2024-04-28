use app_core::Particle;
use app_core::ParticleApi;
use app_core::PluginResult;
use app_core::api::Plugin;
use json::JsonValue;
use crate::json_converter::build_update_func;
use crate::json_converter::to_plugin_result;

pub struct JSPlugin
{
    update:Box<dyn Fn(&JSPlugin, Particle, &mut ParticleApi)>,
    plugin_data: PluginResult,
    json: JsonValue
}

impl JSPlugin
{
    pub fn new(json: &str) -> Result<JSPlugin, String>
    {
        let json_result = json::parse(json);

        match json_result
        {
            Ok(json_value) => 
            {
                let plugin_data = to_plugin_result(&json_value)?;
                let update_func = build_update_func(&json_value, None).unwrap_or(Box::new(|_, _, _| {}));

                Ok(
                    JSPlugin
                    {
                        update: update_func,
                        plugin_data: plugin_data,
                        json: json_value
                    }
                )
            },
            Err(error) => 
            {
                return Err(format!("Error parsing JSON: {}", error).to_string());
            },
        }
    }

    pub fn refresh(&mut self, json: &str) -> Result<(), String>
    {
        let json_result = json::parse(json);

        match json_result
        {
            Ok(json_value) => 
            {
                self.plugin_data = to_plugin_result(&json_value)?;
                let update = build_update_func(&json_value, None);
                
                if update.is_ok()
                {
                    self.update = update.unwrap();
                    // We don't want to save this if the new update is not correct
                    // otherwise we would run on problems on the notify callback
                    self.json = json_value;
                }
                // I should update the json with everything except the update function here if update is not ok
                // the other data is always ok but it's already saved in plugin_data so I'll leave it like this for now.

                Ok(())
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
        }
    }

    fn on_plugin_changed(&mut self, api: &ParticleApi) {
        self.update = build_update_func(&self.json, Some(api)).unwrap_or(Box::new(|_, _, _| {}));   
    }
}