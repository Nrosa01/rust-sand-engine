use egui_macroquad::macroquad::color::BLACK;
use egui_macroquad::macroquad::input::is_key_pressed;
use egui_macroquad::macroquad::input::KeyCode;
use egui_macroquad::macroquad::window::clear_background;

use crate::pop_command;
use crate::Brush;
#[cfg(not(target_family = "wasm"))]
use crate::Debug;
use crate::Entity;
use crate::MessageQueue;
use crate::Universe;

pub struct State {
    pub entities: Vec<Box<dyn Entity>>,
}

// Make universe implement Entity and process as a single entity, State just holds all the entities
// Move selected plugin to brush or to simulation idk

impl State {
    pub fn new() -> State {
        let mut entities:Vec<Box<dyn Entity>>  = vec![
            Box::new(Universe::new()),
            #[cfg(not(target_family = "wasm"))]
            Box::new(Debug::new()),
            Box::new(MessageQueue::new()),
            Box::new(Brush::new()),
        ];

        for entity in entities.iter_mut() {
            entity.init();
        }

        State {
            entities: entities,
        }
    }

    pub fn process_commands(&mut self) {
        let mut command = pop_command();

        while let Some(c) = command {
            for entity in self.entities.iter_mut() {
                entity.receive_command(&c);
            }

            command = pop_command();
        }
    }

    pub fn process(&mut self) {
        self.process_commands();

        self.handle_input();
        self.update();

        clear_background(BLACK);

        self.draw();
        self.ui();
    }

    pub fn should_quit(&self) -> bool {
        return is_key_pressed(KeyCode::Escape);
    }

    fn handle_input(&mut self) {
        for entity in self.entities.iter_mut() {
            entity.handle_input();
        }
    }

    fn update(&mut self) {
        for entity in self.entities.iter_mut() {
            entity.update();
        }
    }

    fn draw(&mut self) {
        for entity in self.entities.iter() {
            entity.draw();
        }
    }

    fn ui(&mut self) {
        
        #[cfg(not(target_family = "wasm"))]
        {    
            egui_macroquad::ui(|egui_ctx| {
                for entity in self.entities.iter_mut() {
                    entity.ui(egui_ctx);
                }
            });
            
            egui_macroquad::draw();
        }
    }
}