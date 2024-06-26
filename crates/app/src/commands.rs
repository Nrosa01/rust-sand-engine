use std::collections::VecDeque;

use app_core::Simulation;

#[allow(unused)]
pub enum Command {
    NewPlugin(String),
    RemovePlugin(u8),
    Debug((String, f32)),
    CanvasSize(u32),
    SetMouseHidden(bool),
    Pause(bool),
    SetBrushSize(isize),
    ParticleSelected(u8),
    StepSimulation,
    SimulationMethod(Box<dyn Fn(&mut Simulation)>),
    NewBackgroundColor([u8; 4]),
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

#[allow(unused)]
pub fn add_dbg(data: (&str, f32)) {
    push_command(Command::Debug((data.0.to_string(), data.1)));
}