use app_core::{ ParticleApi, PluginResult };
use json::JsonValue;

use crate::{blocks::Actions, plugins::JSPlugin};

fn get_color(json: &JsonValue) -> [u8; 4] {
    if json.is_number() {
        let hex = json.as_u32().unwrap();
        let r = ((hex >> 16) & 0xFF) as u8;
        let g = ((hex >> 8) & 0xFF) as u8;
        let b = (hex & 0xFF) as u8;
        let a = 255;
        [r, g, b, a]
    } else if json.is_object() {
        let r = json["r"].as_u8().unwrap_or_default();
        let g = json["g"].as_u8().unwrap_or_default();
        let b = json["b"].as_u8().unwrap_or_default();
        let a = json["a"].as_u8().unwrap_or(255);
        [r, g, b, a]
    } else if json.is_array() {
        let r = json[0].as_u8().unwrap_or_default();
        let g = json[1].as_u8().unwrap_or_default();
        let b = json[2].as_u8().unwrap_or_default();
        let a = json[3].as_u8().unwrap_or(255);
        [r, g, b, a]
    } else {
        [0, 0, 0, 0]
    }
}


pub fn to_plugin_result(json: &JsonValue) -> Result<PluginResult, String> {
    let name = json["name"].as_str().ok_or("name")?;
    let color = get_color(&json["color"]);
    let color2 = get_color(&json["color2"]);

    if json["color"].is_empty() || json["color"].is_null() || json["color2"].is_empty() || json["color2"].is_null(){
        return Err("color was empty or null".to_string());
    }

    Ok(PluginResult {
        name: name.to_string(),
        color: color.into(),
        color2: color2.into(),
    })
}

#[rustfmt::skip]
pub fn build_update_func(json: &JsonValue, api: Option<&ParticleApi>) -> Result<Box<dyn Fn(&JSPlugin, &mut ParticleApi)>, String>
{
    if api.is_none()
    {
        return Err("API is none".to_string());
    }
    
    let update_str = &json["update"];

    if update_str.is_empty() || update_str.is_null() {
        return Err("update was empty or null".to_string());
    }

    let update_str = update_str.to_string();

    let blocks: Vec<Actions> = serde_json::from_str(&update_str).map_err(|err| err.to_string())?;

    let func_vec = blocks
        .iter()
        .map(|block| block.to_func(api.unwrap()))
        .collect::<Vec<_>>();

    Ok(Box::new(move |plugin, api| {
        func_vec.iter().for_each(|func| func(plugin, api));
    }))
}
