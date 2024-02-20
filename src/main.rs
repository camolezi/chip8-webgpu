use chip8_webgpu::{
    chip8::{
        decoder::bit_manipulation::group_in_raw_instructions,
        disassembler::disassemble::disassemble_raw_instructions,
    },
    utilities::storage::{self, write_disassembled_to_file},
};
use inquire::Select;

fn main() {
    let execution_options: Vec<&str> = vec!["Disassemble", "Exit"];
    let selected_execution = Select::new("", execution_options)
        .prompt()
        .expect("Error reading user execution option");

    let rom_data = storage::read_rom("ibm_logo").expect("Unable to load rom");

    let instructions = group_in_raw_instructions(&rom_data);

    let disassembled_instructions = disassemble_raw_instructions(&instructions);

    write_disassembled_to_file("ibm_logo", &disassembled_instructions)
        .expect("was not able to write file")
}
