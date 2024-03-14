use crate::chip8::{
    instructions::base_instruction::IsInstruction, machine_state::state::Chip8VMState,
};

#[derive(Debug)]
pub struct ClearScreenInstruction {}

impl IsInstruction for ClearScreenInstruction {
    fn mnemonic_name(&self) -> &'static str {
        "CLR"
    }

    fn parameters_str(&self) -> String {
        "".to_string()
    }

    fn execute(&self, vm_state: &mut Chip8VMState) {
        vm_state.screen.clean_screen();
    }
}
