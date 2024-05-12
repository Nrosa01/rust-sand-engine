#[cfg(not(target_family = "wasm"))]
use egui_macroquad::egui::Context;

use crate::Command;

pub trait Entity
{
    fn init(&mut self){}
    fn receive_command(&mut self, _: &Command){}
    fn handle_input(&mut self){}
    fn update(&mut self){}
    fn draw(&self){}

    #[cfg(not(target_family = "wasm"))]
    fn ui(&mut self, _: &Context) {}
}