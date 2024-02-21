use crate::chip8::basic_types::Chip8Address;

use super::{
    memory::Chip8Memory, program_counter::ProgramCounter, registers::Chip8Registers,
    screen::Chip8Display, timers::Chip8Timers,
};

#[derive(Default)]
pub struct Chip8VMState {
    pub memory: Chip8Memory,
    pub registers: Chip8Registers,
    pub program_counter: ProgramCounter,
    pub screen: Chip8Display,
    pub timers: Chip8Timers,

    stack: Vec<Chip8Address>,
}
