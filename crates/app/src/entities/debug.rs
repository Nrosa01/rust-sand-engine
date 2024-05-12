use egui_macroquad::macroquad::input::{is_key_pressed, KeyCode};

use crate::{push_command, Command, Entity };

pub struct Debug {}

impl Debug {
    pub fn new() -> Self {
        Debug {}
    }
}

impl Entity for Debug {
    
    fn handle_input(&mut self) {
        if is_key_pressed(KeyCode::P) {
            let data = std::fs::read_to_string("data.json").expect("Unable to read file");
            let data2 = std::fs::read_to_string("replicant.json").expect("Unable to read file");
            push_command(Command::NewPlugin(data));
            push_command(Command::NewPlugin(data2));
        }
    }
}
