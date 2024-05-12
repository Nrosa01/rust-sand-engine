use app_core::ParticleApi;
use app_core::PluginResult;
use app_core::api::Plugin;
use serde::*;
use crate::blocks::Actions;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct JSPluginData {
    pub name: String,
    pub color: [u8; 3],
    pub color2: [u8; 3],
    pub update: Vec<Actions>,
}


pub struct JSPlugin
{
    update:Box<dyn Fn(&JSPlugin, &mut ParticleApi)>,
    plugin_data: JSPluginData,
}

impl JSPlugin
{
    pub fn new(json: &str) -> Result<JSPlugin, serde_json::Error>
    {
        let data = serde_json::from_str(json)?;

       Ok(JSPlugin{
           update: Box::new(|_, _| {}),
           plugin_data: data
       })
    }
}


impl Plugin for JSPlugin
{
    fn update(&self, api: &mut ParticleApi)
    {
        (self.update)(self, api);
    }
    
    fn register(&mut self) -> app_core::PluginResult {
        PluginResult{
            name: self.plugin_data.name.clone(),
            color: self.plugin_data.color.into(),
            color2: self.plugin_data.color2.into()
        }
    }

    fn on_plugin_changed(&mut self, api: &ParticleApi) {
        let func_vec = self.plugin_data.update
            .iter()
            .map(|block| block.to_func(api))
            .collect::<Vec<_>>();
        
        self.update = Box::new(move |plugin, api| {
            func_vec.iter().for_each(|func| func(plugin, api));
        });
    }
}