use super::configuration::{CHIP8_SCREEN_HEIGHT, CHIP8_SCREEN_WIDTH};
use crate::chip8::basic_types::{Byte, ExpandedByte};

type Chip8DisplayPixel = Byte;

pub struct Chip8Display {
    //change this to use 8 pixel group (maybe not, is going to be difficult to ...)
    pixel_data: [[Chip8DisplayPixel; CHIP8_SCREEN_WIDTH]; CHIP8_SCREEN_HEIGHT],
}

impl Default for Chip8Display {
    fn default() -> Self {
        Self {
            pixel_data: [[0; CHIP8_SCREEN_WIDTH]; CHIP8_SCREEN_HEIGHT],
        }
    }
}

impl Chip8Display {
    pub fn clean_screen(&mut self) {
        self.pixel_data.iter_mut().for_each(|row| row.fill(0));
    }

    pub fn get_screen_state(
        &self,
    ) -> &[[Chip8DisplayPixel; CHIP8_SCREEN_WIDTH]; CHIP8_SCREEN_HEIGHT] {
        &self.pixel_data
    }

    pub fn draw_sprite(&mut self, x: Byte, y: Byte, sprite_data: &[ExpandedByte]) -> Byte {
        let usize_x = x as usize;
        let usize_y = y as usize;

        let mut collision = 0;
        sprite_data
            .iter()
            .enumerate()
            .for_each(|(index_y, expanded_byte)| {
                expanded_byte
                    .iter()
                    .enumerate()
                    .for_each(|(index_x, &bit)| {
                        let current_pixel =
                            &mut self.pixel_data[usize_y + index_y][usize_x + index_x];

                        if *current_pixel & bit == 1 {
                            collision = 1;
                        }
                        *current_pixel ^= bit;
                    })
            });

        collision
    }
}
