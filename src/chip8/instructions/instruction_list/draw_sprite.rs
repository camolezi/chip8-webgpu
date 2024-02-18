use crate::chip8::{basic_types::Byte, instructions::base_instruction::IsInstruction};

#[derive(Debug)]
pub struct DrawSprite {
    pub x_position_register: Byte,
    pub y_position_register: Byte,
    pub sprite_size: Byte,
}

impl IsInstruction for DrawSprite {
    fn full_name(&self) -> &'static str {
        "Draw Sprite"
    }

    fn mnemonic_name(&self) -> &'static str {
        "DRW"
    }

    fn parameters_str(&self) -> String {
        format!(
            "V{:x}, V{:x}, {:x}",
            self.x_position_register, self.y_position_register, self.sprite_size
        )
    }

    fn execute(&self) {
        todo!()
    }
}
