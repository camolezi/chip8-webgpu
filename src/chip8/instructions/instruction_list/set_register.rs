use crate::chip8::{
    basic_types::Byte, instructions::base_instruction::IsInstruction,
    machine_state::state::Chip8VMState,
};

#[derive(Debug)]
pub struct SetRegisterInstruction {
    pub register_number: Byte,
    pub data: Byte,
}

impl IsInstruction for SetRegisterInstruction {
    fn mnemonic_name(&self) -> &'static str {
        "SET"
    }

    fn parameters_str(&self) -> String {
        format!("V{}, {:x}", self.register_number, self.data)
    }

    fn execute(&self, vm_state: &mut Chip8VMState) {
        vm_state
            .registers
            .set_data_register(self.register_number, self.data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_register_execute() {
        let mut vm_state = Chip8VMState::default();

        let register_number = 2;
        let data = 51;

        let instruction = SetRegisterInstruction {
            register_number,
            data,
        };
        instruction.execute(&mut vm_state);

        assert_eq!(vm_state.registers.get_data_register(register_number), data);
    }
}
