use super::configuration::CHIP8_NUMBER_OF_DATA_REGISTERS;
use crate::chip8::basic_types::{Byte, Chip8Address};

pub type Chip8DataRegisters = [Byte; CHIP8_NUMBER_OF_DATA_REGISTERS];

#[derive(Default)]
pub struct Chip8Registers {
    data: Chip8DataRegisters,
    address: Chip8Address,
}

impl Chip8Registers {
    pub fn set_data_register(&mut self, register_number: Byte, data: Byte) {
        self.data[register_number as usize] = data;
    }

    pub fn get_data_register(&self, register_number: Byte) -> Byte {
        self.data[register_number as usize]
    }

    pub fn set_address_register(&mut self, data: Chip8Address) {
        self.address = data;
    }

    pub fn get_address_register(&self) -> Chip8Address {
        self.address
    }
}
