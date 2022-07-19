#![doc(hidden)]
use sdl2::keyboard::Keycode as Key;

//Window
pub const WIDTH: u32 = 64;
pub const HEIGHT: u32 = 32;
pub const TITLE: &str = "Ace emulator";
//pub const MULTIPLIER: u32 = 10;

//Chip8 Specifications
pub const KEYPAD: u8 = 16;
pub const MEMORY: u16 = 4096;
pub const TOTAL_DATA_REGISTERS: u8 = 16;
pub const TOTAL_STACK_SIZE: u8 = 16;
pub const MEM_START: u16 = 0x200;

//Default Font
pub const DEFAULT_CHAR_SET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, //0
    0x20, 0x60, 0x20, 0x20, 0x70, //1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, //2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, //3
    0x90, 0x90, 0xF0, 0x10, 0x10, //4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, //5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, //6
    0xF0, 0x10, 0x20, 0x40, 0x40, //7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, //8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, //9
    0xF0, 0x90, 0xF0, 0x90, 0x90, //A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, //B
    0xF0, 0x80, 0x80, 0x80, 0xF0, //C
    0xE0, 0x90, 0x90, 0x90, 0xE0, //D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, //E
    0xF0, 0x80, 0xF0, 0x80, 0x80, //F
];

//Default Keymap
#[rustfmt::skip]
pub const KEY_MAP: [Key; KEYPAD as usize] = [
    Key::Num1, Key::Num2, Key::Num3, Key::Num4,
    Key::Q, Key::W, Key::E, Key::R,
    Key::A, Key::S, Key::D, Key::F,
    Key::Z, Key::X, Key::C, Key::V
    ];

//Tweaks

//Color Tweaks
pub struct Color(pub f32, pub f32, pub f32);

#[rustfmt::skip]
pub const PALETTE: [Color; 10] = [
    Color(1.0, 0.0, 0.0), //RED
    Color(0.0, 1.0, 0.0), //GREEN
    Color(0.0, 0.0, 1.0), //BLUE
    Color(0.0, 0.0, 0.0), //BLACK
    Color(1.0, 1.0, 1.0), //WHITE
    Color(1.0, 0.4, 0.6), //LIGHT PINK
    Color(0.5, 0.3, 1.0), //LIGHT BLUE
    Color(0.5411, 0.1686, 0.8862), //BLUE VIOLET
    Color(1.0, 1.0, 0.0), //YELLOW
    Color(0.4862,0.9882,0.0)  //LAWN GREEN
];

//Shader location
pub const VERTEX_LOC: &str = "config/shader/triangle.vert";
pub const FRAGMENT_LOC: &str = "config/shader/triangle.frag";
