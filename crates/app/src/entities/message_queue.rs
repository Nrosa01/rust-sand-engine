use egui_macroquad::macroquad::{self, time::get_frame_time, window::screen_height};

use crate::Entity;

type Message = (String, f32);

pub struct MessageQueue
{
    messages: Vec<Message>,
}

impl MessageQueue
{
    pub fn new() -> Self
    {
        MessageQueue
        {
            messages: Vec::new(),
        }
    }
}

impl Entity for MessageQueue
{
    fn update(&mut self)
    {
        self.messages.retain(|(_, time)| *time > 0.0);
        self.messages.iter_mut().for_each(|(_, time)| *time -= get_frame_time());
    }

    fn receive_command(&mut self, command: &crate::Command) {
        match command
        {
            crate::Command::Debug(message) => self.messages.push(message.clone()),
            _ => {}
        }
    }

    fn draw(&self)
    {
        let mut color = macroquad::color::WHITE;
        for (i, (message, time)) in self.messages.iter().enumerate()
        {
            color.a = (time / 2.0).min(1.0);
            macroquad::text::draw_text(&message, 10.0, screen_height() - 30.0 - i as f32 * 20.0, 20.0, color);
        }
    }
}