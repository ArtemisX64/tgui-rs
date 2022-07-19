//!# Chip8 Display
//!It implements the chip8 display
//!```
//!use super::display::Display;
//!
//!let display = Display::new();
//!```
use super::cfg;

pub struct Display {
    pixels: [[bool; cfg::WIDTH as usize]; cfg::HEIGHT as usize],
}

impl Display {
    ///Initializes Display
    pub fn new() -> Self {
        Display {
            pixels: [[false; cfg::WIDTH as usize]; cfg::HEIGHT as usize],
        }
    }

    ///Checks if the pixel is on
    pub fn is_set(&self, x: usize, y: usize) -> bool {
        self.pixels[y][x]
    }

    ///**Draws Pixel**
    ///
    ///Pixel is xored into the screen
    pub fn draw_pixels(&mut self, sprite: &[u8], x: usize, y: usize) -> bool {
        let mut collision = false;
        for (ly, pixel) in sprite.iter().enumerate() {
            for lx in 0usize..8usize {
                if (pixel & (0b10000000 >> lx)) == 0 {
                    continue;
                }
                collision =
                    self.pixels[(y + ly) % cfg::HEIGHT as usize][(x + lx) % cfg::WIDTH as usize];
                self.pixels[(y + ly) % cfg::HEIGHT as usize][(x + lx) % cfg::WIDTH as usize] ^=
                    true;
            }
        }
        collision
    }

    ///Clears the display
    pub fn clear_screen(&mut self) {
        self.pixels = [[false; cfg::WIDTH as usize]; cfg::HEIGHT as usize];
    }
}
