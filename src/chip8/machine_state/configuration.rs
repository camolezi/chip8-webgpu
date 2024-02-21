use crate::chip8::basic_types::Chip8Address;

pub const CHIP8_MEMORY_SIZE: usize = 4096;
pub const CHIP8_NUMBER_OF_DATA_REGISTERS: usize = 16;
pub const CHIP8_STACK_INITIAL_SIZE: usize = 20;

//0x200
pub const MEMORY_ROM_START_POSITION: usize = 512;

pub const PROGRAM_COUNTER_START_POSITION: Chip8Address = 512;

pub const CHIP8_SCREEN_HEIGHT: usize = 32; // 4 bytes, 32 pixels
pub const CHIP8_SCREEN_WIDTH: usize = 64; // 8 bytes, 64 pixels

pub const CHIP8_SPRITE_FIXED_WIDTH: usize = 8;
