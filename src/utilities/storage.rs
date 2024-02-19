use std::path::Path;

const ROMS_DIR_PATH: &'static str = "roms";

pub fn read_rom(rom_name: &str) -> Result<Vec<u8>, std::io::Error> {
    let rom_path = Path::new(ROMS_DIR_PATH).join(rom_name.to_owned() + ".ch8");
    println!("Reading ROM at {:#?}", rom_path);
    std::fs::read(rom_path)
}

const DISASSEMBLED_DIR_PATH: &'static str = "disassembled";

pub fn write_disassembled_to_file(
    file_name: &str,
    disassembled_instructions: &Vec<String>,
) -> Result<(), std::io::Error> {
    let disassembled_path = Path::new(DISASSEMBLED_DIR_PATH).join(file_name.to_owned() + ".txt");
    println!("Writing disassembled_path  rom at {:#?}", disassembled_path);

    let file_content = disassembled_instructions
        .iter()
        .fold(String::new(), |acc, x| acc + x + "\n");

    std::fs::write(disassembled_path, file_content)
}
