use crate::chip8::basic_types::Byte;

#[derive(Default)]
pub struct Chip8Timers {
    delay: Byte,
    sound: Byte,
}
