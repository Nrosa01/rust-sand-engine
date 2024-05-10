use crate::*;

#[no_mangle]
pub extern "C" fn receive_json_plugin(data: sapp_jsutils::JsObject) {

    if data.is_nil() {
        return;
    }

    let mut buffer = String::new();
    data.to_string(&mut buffer);

    push_command(Command::NewPlugin(buffer));
}

#[no_mangle]
pub extern "C" fn pause(data: sapp_jsutils::JsObject) {

    if data.is_nil() {
        return;
    }

    let mut buffer = String::new();
    data.to_string(&mut buffer);

    let paused = buffer.parse().unwrap_or(false);
    push_command(Command::Pause(paused));
    if buffer.is_empty() {
        add_dbg(("Pause command received with empty data", 2.0));
    } else {
        add_dbg((&format!("Pause command received with data: {}", buffer), 5.0));
    }
}


#[no_mangle]
pub extern "C" fn clear() {
    push_command(Command::Clear);
    add_dbg(("Clear command received", 2.0));
}

#[no_mangle]
pub extern "C" fn resize_simulation(data: sapp_jsutils::JsObject) {

    if data.is_nil() {
        return;
    }

    let mut buffer = String::new();
    data.to_string(&mut buffer);

    let size = buffer.parse();

    match size {
        Ok(size) => {
            push_command(Command::CanvasSize(size));
            add_dbg((&format!("Resize command received with data: {}", size), 5.0));
        }
        Err(_) => {
            add_dbg((&format!("Resize command received with data: {}", buffer), 2.0));
        }
    }
}

#[no_mangle]
pub extern "C" fn select_particle(data: sapp_jsutils::JsObject) {

    if data.is_nil() {
        return;
    }

    let mut buffer = String::new();
    data.to_string(&mut buffer);

    let id = buffer.parse();

    match id {
        Ok(id) => {
            push_command(Command::ParticleSelected(id));
            add_dbg((&format!("Select particle command received with data: {}", id), 5.0));
        }
        Err(_) => {
            add_dbg(("Select particle command received with invalid data", 2.0));
        }
    }
}

#[no_mangle]
pub extern "C" fn remove_plugin(data: sapp_jsutils::JsObject) {

    if data.is_nil() {
        return;
    }

    let mut buffer = String::new();
    data.to_string(&mut buffer);
    let buffer = buffer.parse().unwrap(); // I don't manage errors here because this shuouldnt fail at all

    add_dbg((&format!("Remove plugin command received with data: {}", buffer), 5.0));
    push_command(Command::RemovePlugin(buffer));
}

#[no_mangle]
pub extern "C" fn set_mouse_hidden(data: sapp_jsutils::JsObject) {

    if data.is_nil() {
        return;
    }

    let mut buffer = String::new();
    data.to_string(&mut buffer);

    let hidden = buffer.parse().unwrap_or(false);
    push_command(Command::SetMouseHidden(hidden));
    if buffer.is_empty() {
        add_dbg(("Set mouse hidden command received with empty data", 2.0));
    } else {
        add_dbg((&format!("Set mouse hidden command received with data: {}", buffer), 5.0));
    }
}

#[no_mangle]
pub extern "C" fn set_brush_size(data: sapp_jsutils::JsObject) {

    if data.is_nil() {
        return;
    }

    let mut buffer = String::new();
    data.to_string(&mut buffer);

    let size = buffer.parse();

    match size {
        Ok(size) => {
            push_command(Command::SetBrushSize(size));
            add_dbg((&format!("Set brush size command received with data: {}", size), 5.0));
        }
        Err(_) => {
            add_dbg((&format!("Set brush size command received with data: {}", buffer), 2.0));
        }
    }
}

#[no_mangle]
pub extern "C" fn step_simulation() {
    push_command(Command::StepSimulation);
    add_dbg(("Step simulation command received", 5.0));
}


#[no_mangle]
pub fn pixel_creator_api_crate_version() -> u32
{
    (1 << 24) + (0 << 16) + 0
}

extern "C" {
    pub fn send_to_js(data: sapp_jsutils::JsObject);
}