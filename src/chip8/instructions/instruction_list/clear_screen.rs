use crate::chip8::instructions::base_instruction::IsInstruction;

#[derive(Debug)]
pub struct ClearScreenInstruction {}

impl IsInstruction for ClearScreenInstruction {
    fn full_name(&self) -> &'static str {
        "Clear Screen"
    }

    fn mnemonic_name(&self) -> &'static str {
        "CLR"
    }

    fn parameters_str(&self) -> String {
        "".to_string()
    }

    fn execute(&self) {
        todo!()
    }
}
