use std::{sync::mpsc, thread, time::Duration};

use chip8_webgpu::{
    chip8::{
        basic_types::RawInstruction,
        decoder::decode::decode_instruction,
        machine_state::{state::Chip8VMState, Chip8DisplayData},
    },
    utilities::{execution_timer::ExecutionTimer, storage},
};

use macroquad::prelude::*;

const PIXEL_SCALE: f32 = 12.0;

//Use 1 or more for grid
const PIXEL_SPACING: f32 = 0.0;

#[macroquad::main("Chip8 Emulator")]
async fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || run_emulation_thread(tx));

    loop {
        let screen_state = rx.recv().unwrap();

        clear_background(GRAY);

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

const FRAMES_PER_SECOND: u64 = 60;
const EMULATION_RUNS_PER_SECOND: u64 = 500;

const FRAME_INTERVAL: Duration = Duration::from_millis(1000 / FRAMES_PER_SECOND);
const EMULATION_INTERVAL: Duration = Duration::from_millis(1000 / EMULATION_RUNS_PER_SECOND);

fn run_emulation_thread(tx: mpsc::Sender<Chip8DisplayData>) {
    let rom_data = storage::read_rom("ibm_logo_2").expect("Unable to load rom");

    let mut vm_state = Chip8VMState::default();
    vm_state.memory.load_rom(&rom_data);

    let mut frame_execution_timer = ExecutionTimer::new(FRAME_INTERVAL);
    let mut emulation_execution_timer = ExecutionTimer::new(EMULATION_INTERVAL);

    let sleep_interval: Duration = std::cmp::min(FRAME_INTERVAL, EMULATION_INTERVAL);

    loop {
        emulation_execution_timer.run(|| {
            let current_instruction_address = vm_state.program_counter.get_pc();
            let current_instruction_data =
                vm_state.memory.read_memory(current_instruction_address, 2);
            vm_state.program_counter.next();

            let decoded_instruction = decode_instruction(RawInstruction(
                current_instruction_data[0],
                current_instruction_data[1],
            ));

            decoded_instruction.execute(&mut vm_state);
        });

        frame_execution_timer.run(|| {
            tx.send(*vm_state.screen.get_screen_state()).unwrap();
        });

        thread::sleep(sleep_interval);
    }
}
