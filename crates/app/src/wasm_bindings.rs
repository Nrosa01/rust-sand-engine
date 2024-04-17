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
            add_dbg(("Resize command received with invalid data", 2.0));
        }
    }
}