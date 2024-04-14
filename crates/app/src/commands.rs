use std::collections::VecDeque;

#[allow(unused)]
pub enum Command {
    NewPlugin(String),
    CanvasSize(u32),
    Clear,
}

pub static mut COMMANDS: VecDeque<Command> = VecDeque::new();

pub fn pop_command() -> Option<Command> {
    unsafe {
        COMMANDS.pop_front()
    }
}

#[allow(unused)]
pub fn push_command(command: Command) {
    unsafe {
        COMMANDS.push_back(command);
    }
}