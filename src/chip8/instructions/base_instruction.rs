pub trait IsInstruction: std::fmt::Debug {
    fn execute(&self);

    fn full_name(&self) -> &'static str;
    fn mnemonic_name(&self) -> &'static str;

    fn parameters_str(&self) -> String;
}

pub type DynamicInstruction = Box<dyn IsInstruction>;
