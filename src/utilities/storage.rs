use std::path::Path;

const ROMS_DIR_PATH: &'static str = "roms";

pub fn read_rom(rom_name: &str) -> Result<Vec<u8>, std::io::Error> {
    let rom_path = Path::new(ROMS_DIR_PATH).join(rom_name.to_owned() + ".ch8");
    println!("Reading ROM at {:#?}", rom_path);
    std::fs::read(rom_path)
}
