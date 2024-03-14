use crate::chip8::machine_state::state::Chip8VMState;

pub trait IsInstruction: std::fmt::Debug {
    fn execute(&self, vm_state: &mut Chip8VMState);

    fn mnemonic_name(&self) -> &'static str;

    fn parameters_str(&self) -> String;
}

pub type DynamicInstruction = Box<dyn IsInstruction>;
