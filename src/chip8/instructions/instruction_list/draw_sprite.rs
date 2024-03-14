use crate::chip8::{
    basic_types::{Byte, ExpandedByte},
    decoder::bit_manipulation,
    instructions::base_instruction::IsInstruction,
    machine_state::state::Chip8VMState,
};

#[derive(Debug)]
pub struct DrawSprite {
    pub x_position_register: Byte,
    pub y_position_register: Byte,
    pub sprite_size: Byte,
}

impl IsInstruction for DrawSprite {
    fn mnemonic_name(&self) -> &'static str {
        "DRW"
    }

    fn parameters_str(&self) -> String {
        format!(
            "V{:x}, V{:x}, {:x}",
            self.x_position_register, self.y_position_register, self.sprite_size
        )
    }

    fn execute(&self, vm_state: &mut Chip8VMState) {
        let sprite_start_address = vm_state.registers.get_address_register();

        let sprite_data = vm_state
            .memory
            .read_memory(sprite_start_address, self.sprite_size);

        let bit_sprite_data: Vec<ExpandedByte> = sprite_data
            .iter()
            .map(|&byte| bit_manipulation::get_byte_bits(byte))
            .collect();

        let x_position = vm_state
            .registers
            .get_data_register(self.x_position_register);

        let y_position = vm_state
            .registers
            .get_data_register(self.y_position_register);

        let collision = vm_state
            .screen
            .draw_sprite(x_position, y_position, &bit_sprite_data);

        vm_state.registers.set_data_register(15, collision)
    }
}
