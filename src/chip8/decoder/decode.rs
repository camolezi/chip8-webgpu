use super::bit_manipulation::{self, combine_address_nibbles, combine_nibbles};
use crate::chip8::{
    basic_types::RawInstruction,
    instructions::{
        base_instruction::DynamicInstruction,
        instruction_list::{
            add_number_register::AddNumberRegisterInstruction,
            clear_screen::ClearScreenInstruction, copy_register::CopyRegisterInstruction,
            draw_sprite::DrawSprite, jump::JumpInstruction, not_opcode::NotOpcode,
            or_register::OrRegisterInstruction, set_address_register::SetAddressRegister,
            set_register::SetRegisterInstruction,
        },
    },
};

pub fn decode_instruction(
    RawInstruction(first_byte, second_byte): RawInstruction,
) -> DynamicInstruction {
    let (first_nibble, second_nibble) = bit_manipulation::get_byte_nibbles(first_byte);
    let (third_nibble, fourth_nibble) = bit_manipulation::get_byte_nibbles(second_byte);

    match (first_nibble, second_nibble, third_nibble, fourth_nibble) {
        (0x00, 0x00, 0x0e, 0x00) => Box::new(ClearScreenInstruction {}),
        (0x01, _, _, _) => Box::new(JumpInstruction {
            to_address: combine_address_nibbles((second_nibble, third_nibble, fourth_nibble)),
        }),
        (0x06, _, _, _) => Box::new(SetRegisterInstruction {
            register_number: second_nibble,
            data: combine_nibbles((third_nibble, fourth_nibble)),
        }),
        (0x07, _, _, _) => Box::new(AddNumberRegisterInstruction {
            register_number: second_nibble,
            data: combine_nibbles((third_nibble, fourth_nibble)),
        }),
        (0x08, _, _, 0x00) => Box::new(CopyRegisterInstruction {
            register_number: second_nibble,
            copied_register_number: third_nibble,
        }),
        (0x08, _, _, 0x01) => Box::new(OrRegisterInstruction {
            register_x: second_nibble,
            register_y: third_nibble,
        }),
        (0x0a, _, _, _) => Box::new(SetAddressRegister {
            address: combine_address_nibbles((second_nibble, third_nibble, fourth_nibble)),
        }),
        (0x0d, _, _, _) => Box::new(DrawSprite {
            x_position_register: second_nibble,
            y_position_register: third_nibble,
            sprite_size: fourth_nibble,
        }),
        _ => Box::new(NotOpcode {}),
    }
}

pub fn decode_instructions(instructions: &[RawInstruction]) -> Vec<DynamicInstruction> {
    instructions
        .iter()
        .map(|inst| decode_instruction(*inst))
        .collect()
}
