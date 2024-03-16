use chip8_webgpu::chip8::{
    basic_types::{Byte, RawInstruction},
    decoder::{bit_manipulation::combine_nibbles, decode::decode_instruction},
    machine_state::state::Chip8VMState,
};

#[derive(Default)]
pub struct InstructionTest {
    vm_state: Chip8VMState,
}

impl InstructionTest {
    pub fn setup<T>(&mut self, f: T)
    where
        T: FnOnce(&mut Chip8VMState),
    {
        f(&mut self.vm_state);
    }

    pub fn execute(&mut self, raw_instruction: RawInstruction) {
        let decoded_instruction = decode_instruction(raw_instruction);
        decoded_instruction.execute(&mut self.vm_state);
    }

    pub fn assert<T>(&mut self, f: T)
    where
        T: FnOnce(&mut Chip8VMState),
    {
        f(&mut self.vm_state);
    }
}

pub fn create_raw_instruction(
    (nibble_1, nibble_2): (Byte, Byte),
    (nibble_3, nibble_4): (Byte, Byte),
) -> RawInstruction {
    RawInstruction(
        combine_nibbles((nibble_1, nibble_2)),
        combine_nibbles((nibble_3, nibble_4)),
    )
}

pub fn create_raw_instruction_w_data(
    (nibble_1, nibble_2): (Byte, Byte),
    data: Byte,
) -> RawInstruction {
    RawInstruction(combine_nibbles((nibble_1, nibble_2)), data)
}
