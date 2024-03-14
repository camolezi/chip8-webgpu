use crate::chip8::{
    basic_types::Chip8Address, instructions::base_instruction::IsInstruction,
    machine_state::state::Chip8VMState,
};

#[derive(Debug)]
pub struct JumpInstruction {
    pub to_address: Chip8Address,
}

impl IsInstruction for JumpInstruction {
    fn mnemonic_name(&self) -> &'static str {
        "JUMP"
    }

    fn parameters_str(&self) -> String {
        format!("0x{:x}", self.to_address)
    }

    fn execute(&self, vm_state: &mut Chip8VMState) {
        vm_state.program_counter.set_pc(self.to_address);
    }
}
