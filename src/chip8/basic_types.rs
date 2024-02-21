pub type Chip8Address = u16;
pub type Byte = u8;

pub type Bit = u8;

pub type ExpandedByte = [Bit; 8];

#[derive(Debug, Clone, Copy)]
pub struct RawInstruction(pub Byte, pub Byte);
