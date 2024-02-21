use crate::chip8::basic_types::Chip8Address;

use super::configuration::PROGRAM_COUNTER_START_POSITION;

pub struct ProgramCounter {
    counter: Chip8Address,
}

impl Default for ProgramCounter {
    fn default() -> Self {
        Self {
            counter: PROGRAM_COUNTER_START_POSITION,
        }
    }
}

impl ProgramCounter {
    pub fn set_pc(&mut self, data: Chip8Address) {
        self.counter = data;
    }

    pub fn get_pc(&self) -> Chip8Address {
        self.counter
    }
}
