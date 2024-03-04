use chip8_webgpu::{
    chip8::{
        basic_types::RawInstruction, decoder::decode::decode_instruction,
        machine_state::state::Chip8VMState,
    },
    utilities::storage,
};

use macroquad::prelude::*;

const PIXEL_SCALE: f32 = 12.0;

//Use 1 or more for grid
const PIXEL_SPACING: f32 = 0.0;

#[macroquad::main("Chip8 Emulator")]
async fn main() {
    let rom_data = storage::read_rom("ibm_logo_2").expect("Unable to load rom");

    let mut vm_state = Chip8VMState::default();
    vm_state.memory.load_rom(&rom_data);

   // let (tx, rx) = mpsc::channel();

    loop {
        let current_instruction_address = vm_state.program_counter.get_pc();
        let current_instruction_data = vm_state.memory.read_memory(current_instruction_address, 2);
        vm_state.program_counter.next();

        let decoded_instruction = decode_instruction(RawInstruction(
            current_instruction_data[0],
            current_instruction_data[1],
        ));

        decoded_instruction.execute(&mut vm_state);

        clear_background(GRAY);

        let screen_state = vm_state.screen.get_screen_state();

        for (y_index, pixel_row) in screen_state.iter().enumerate() {
            for (x_index, &pixel) in pixel_row.iter().enumerate() {
                let pixel_x = PIXEL_SCALE * (x_index as f32);
                let pixel_y = PIXEL_SCALE * (y_index as f32);

                if pixel == 1 {
                    draw_rectangle(
                        pixel_x,
                        pixel_y,
                        PIXEL_SCALE - PIXEL_SPACING,
                        PIXEL_SCALE - PIXEL_SPACING,
                        WHITE,
                    );
                } else {
                    draw_rectangle(
                        pixel_x,
                        pixel_y,
                        PIXEL_SCALE - PIXEL_SPACING,
                        PIXEL_SCALE - PIXEL_SPACING,
                        BLACK,
                    );
                }
            }
        }

        next_frame().await
    }

    // let instructions = group_in_raw_instructions(&rom_data);

    //     let execution_options: Vec<&str> = vec!["Disassemble", "Exit"];
    //     let selected_execution = Select::new("", execution_options)
    //         .prompt()
    //         .expect("Error reading user execution option");
    //     let disassembled_instructions = disassemble_raw_instructions(&instructions);

    //    write_disassembled_to_file("ibm_logo", &disassembled_instructions)
    //        .expect("was not able to write file")
}

fn debug_print_pixel() {}
