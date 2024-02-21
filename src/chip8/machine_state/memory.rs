use crate::chip8::basic_types::{Byte, Chip8Address};

use super::configuration::{CHIP8_MEMORY_SIZE, MEMORY_ROM_START_POSITION};

pub struct Chip8Memory {
    mem_array: [Byte; CHIP8_MEMORY_SIZE],
}

impl Default for Chip8Memory {
    fn default() -> Self {
        Self {
            mem_array: [0; CHIP8_MEMORY_SIZE],
        }
    }
}

impl Chip8Memory {
    pub fn load_rom(&mut self, rom_bytes: &[Byte]) {
        let mem_rom_slice = &mut self.mem_array
            [MEMORY_ROM_START_POSITION..MEMORY_ROM_START_POSITION + rom_bytes.len()];
        mem_rom_slice.copy_from_slice(rom_bytes);
    }

    pub fn read_memory(&self, start_address: Chip8Address, bytes: Byte) -> &[Byte] {
        let usize_start = start_address as usize;
        let usize_bytes = bytes as usize;

        &self.mem_array[usize_start..usize_start + usize_bytes]
    }
}
