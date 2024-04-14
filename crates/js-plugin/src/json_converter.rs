use app_core::{Particle, ParticleApi, PluginResult, Vec2};
use json::JsonValue;

use crate::blocks::Blocks;

fn get_color(json: &JsonValue) -> [u8; 4]
{
    if json.is_number()
    {
        let hex = json.as_u32().unwrap();
        let r = ((hex >> 16) & 0xFF) as u8;
        let g = ((hex >> 8) & 0xFF) as u8;
        let b = (hex & 0xFF) as u8;
        let a = 255;
        [r, g, b, a]
    }
    else if json.is_object()
    {
        let r = json["r"].as_u8().unwrap_or_default();
        let g = json["g"].as_u8().unwrap_or_default();
        let b = json["b"].as_u8().unwrap_or_default();
        let a = json["a"].as_u8().unwrap_or(255);
        [r, g, b, a]
    }
    else if json.is_array()
    {
        let r = json[0].as_u8().unwrap_or_default();
        let g = json[1].as_u8().unwrap_or_default();
        let b = json[2].as_u8().unwrap_or_default();
        let a = json[3].as_u8().unwrap_or(255);
        [r, g, b, a]
    }
    else
    {
        [0, 0, 0, 0]
    }
}

fn get_alpha(json: &JsonValue) -> Vec2
{
    if json.is_number()
    {
        let alpha = json.as_f64().unwrap();
        Vec2 { x: alpha as f32, y: alpha as f32 }
    }
    else if json.is_array()
    {
        let min = json[0].as_f64().unwrap();
        let max = json[1].as_f64().unwrap();
        Vec2 { x: min as f32, y: max as f32 }
    }
    else
    {
        PluginResult::default().alpha
    }
}

fn get_extra(json: &JsonValue) -> Vec2
{
    if json.is_number()
    {
        let extra = json.as_f64().unwrap();
        Vec2 { x: extra as f32, y: extra as f32 }
    }
    else if json.is_array()
    {
        let min = json[0].as_f64().unwrap();
        let max = json[1].as_f64().unwrap();
        Vec2 { x: min as f32, y: max as f32 }
    }
    else
    {
        PluginResult::default().extra
    }
}

pub fn to_plugin_result(json: &JsonValue) -> Result<PluginResult, String>
{
    let name = json["name"].as_str().ok_or("name")?;
    let color = get_color(&json["color"]);
    let alpha = get_alpha(&json["alpha"]);
    let extra = get_extra(&json["extra"]);
    let hidden_in_ui = json["hidden_in_ui"].as_bool().unwrap_or(false); // Optional, default false

    if json["color"].is_empty() || json["color"].is_null()
    {
        return Err("color was empty or null".to_string());
    }

    Ok(PluginResult {
        name: name.to_string(),
        color: color.into(),
        alpha: alpha,
        extra: extra,
        hidden_in_ui: hidden_in_ui,
    })
}

pub fn build_update_func(json: &JsonValue) -> Result<Box<dyn Fn(Particle, &mut ParticleApi) -> bool>, String>
{   
    let update_str = json["update"].to_string();
    let serde_json = serde_json::from_str(&update_str).unwrap();
    
    println!("Serde correctly parsed the JSON");
    let blocks: Vec<Blocks> = serde_json;

    // For testing we will print blocks
    println!("Using serde_json:");
    for block in &blocks
    {
        println!("{:?}", block);
    }

    let func_vec = blocks.iter().map(|block| block.to_func()).collect::<Vec<_>>();

    // For testing we will just return an empty function
    Ok(Box::new(move |particle, api| {
        func_vec.iter().all(|func| func(particle, api));
        return true;
    }))
}