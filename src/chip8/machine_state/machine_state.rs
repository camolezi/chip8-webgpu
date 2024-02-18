use crate::chip8::basic_types::{Byte, Chip8Address};

use super::configuration::{
    CHIP8_MEMORY_SIZE, CHIP8_NUMBER_OF_DATA_REGISTERS, CHIP8_STACK_INITIAL_SIZE,
};

pub struct Chip8Chip8MemoryVirtualMachineState {
    memory: Chip8Memory,
    registers: Chip8Registers,
    program_counter: Chip8Address,
    stack: Vec<Chip8Address>,
    timers: Chip8Timers,

    screen: Chip8Display,
}

pub type Chip8Memory = [Byte; CHIP8_MEMORY_SIZE];

pub type Chip8DataRegisters = [Byte; CHIP8_NUMBER_OF_DATA_REGISTERS];

pub struct Chip8Registers {
    data: Chip8DataRegisters,
    address: Chip8Address,
}

pub struct Chip8Timers {
    delay: Byte,
    sound: Byte,
}

impl Default for Chip8Chip8MemoryVirtualMachineState {
    fn default() -> Self {
        Chip8Chip8MemoryVirtualMachineState {
            memory: [0; CHIP8_MEMORY_SIZE],
            registers: Chip8Registers {
                data: [0; CHIP8_NUMBER_OF_DATA_REGISTERS],
                address: 0,
            },
            program_counter: 0,
            stack: vec![0; CHIP8_STACK_INITIAL_SIZE],
            timers: Chip8Timers { delay: 0, sound: 0 },
            screen: Chip8Display {
                pixel_data: [[0; CHIP8_SCREEN_HEIGHT]; CHIP8_SCREEN_WIDTH],
            },
        }
    }
}

type Chip8DisplayPixel = u8;
const CHIP8_SCREEN_HEIGHT: usize = 32;
const CHIP8_SCREEN_WIDTH: usize = 64;

struct Chip8Display {
    pixel_data: [[Chip8DisplayPixel; CHIP8_SCREEN_HEIGHT]; CHIP8_SCREEN_WIDTH],
}
