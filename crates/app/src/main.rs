#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release


use app_core::api::Simulation;
use egui_macroquad::macroquad;
use macroquad::prelude::*;
use std::error::Error;

mod commands;
use commands::*;

mod entities;
use entities::*;

#[cfg(target_family = "wasm")]
mod wasm_bindings;

const WINDOW_WIDTH: i32 = 800;
const WINDOW_HEIGHT: i32 = 800;

fn conf() -> Conf {
    Conf {
        window_title: String::from("Pixel Flow"),
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() -> Result<(), Box<dyn Error>> {
    macroquad::rand::srand(macroquad::miniquad::date::now() as u64);
    let mut state = State::new();

    loop {
        state.process();

        if state.should_quit() {
            break;
        }

        next_frame().await;
    }

    Ok(())
}
