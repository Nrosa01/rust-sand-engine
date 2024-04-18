use app_core::{Particle, ParticleApi, PluginResult, Vec2};
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

fn get_alpha(json: &JsonValue) -> Vec2 {
    if json.is_number() {
        let alpha = json.as_f64().unwrap();
        Vec2 {
            x: alpha as f32,
            y: alpha as f32,
        }
    } else if json.is_array() {
        let min = json[0].as_f64().unwrap();
        let max = json[1].as_f64().unwrap();
        Vec2 {
            x: min as f32,
            y: max as f32,
        }
    } else {
        PluginResult::default().alpha
    }
}

fn get_extra(json: &JsonValue) -> Vec2 {
    if json.is_number() {
        let extra = json.as_f64().unwrap();
        Vec2 {
            x: extra as f32,
            y: extra as f32,
        }
    } else if json.is_array() {
        let min = json[0].as_f64().unwrap();
        let max = json[1].as_f64().unwrap();
        Vec2 {
            x: min as f32,
            y: max as f32,
        }
    } else {
        PluginResult::default().extra
    }
}

pub fn to_plugin_result(json: &JsonValue) -> Result<PluginResult, String> {
    let name = json["name"].as_str().ok_or("name")?;
    let color = get_color(&json["color"]);
    let alpha = get_alpha(&json["alpha"]);
    let extra = get_extra(&json["extra"]);

    if json["color"].is_empty() || json["color"].is_null() {
        return Err("color was empty or null".to_string());
    }

    Ok(PluginResult {
        name: name.to_string(),
        color: color.into(),
        alpha: alpha,
        extra: extra,
    })
}

#[rustfmt::skip]
pub fn build_update_func(json: &JsonValue, api: Option<&ParticleApi>) -> Result<Box<dyn Fn(&JSPlugin, Particle, &mut ParticleApi)>, String>
{
    if api.is_none()
    {
        return Ok(Box::new(|_, _, _| {}));
    }
    
    let update_str = json["update"].to_string();
    let blocks: Vec<Actions> = serde_json::from_str(&update_str).map_err(|err| err.to_string())?;

    let func_vec = blocks
        .iter()
        .map(|block| block.to_func(api.unwrap()))
        .collect::<Vec<_>>();

    Ok(Box::new(move |plugin, particle, api| {
        func_vec.iter().for_each(|func| func(plugin, particle, api));
    }))
}
