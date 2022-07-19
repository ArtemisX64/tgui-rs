//!# Description
//!A Chip8 emulator made using Rust\
//!This library loads and executes chip8 program in
//! - [x] OpenGL
//! - [x] tgui
//!
//!This lib is called using load() in respective binaries\
//!# Exec
//!```
//!use chip8;
//!
//!chip8::load();
//!```

///Backend
mod backend;
mod config;
///Chip8 CPU
mod cpu;
///Chip8 Display
mod display;
///Chip8 Keyboard
mod keyboard;
///Chip8 Memory
mod memory;

use ini::Ini;
use rand::Rng;
use std::convert::TryInto;
use std::fs::File;
use std::io::Read;
use std::thread;
use std::time::Duration;

use backend::opengl;
use config as cfg;

use cpu::Cpu;
use display::Display;
use keyboard::Keyboard;
use memory::Memory;

use opengl::{mesh::Mesh, shader::Shader, window::Window};

///#Ace emulator config
pub struct Config {
    v_shader: String,
    f_shader: String,
    scale: u32,
    bg: &'static cfg::Color,
    pub fg: &'static cfg::Color,
    delay: u16,
    audio: bool,
    cpu: u16,
}

///# Chip8 Struct
/// Implements chip8 hardware
pub struct Chip8 {
    display: Display,
    keyboard: Keyboard,
    memory: Memory,
    cpu: Cpu,
}

impl Chip8 {
    ///Initializes Chip8
    fn new() -> Self {
        Chip8 {
            display: Display::new(),
            keyboard: Keyboard::new(),
            memory: Memory::new(),
            cpu: Cpu::new(),
        }
    }

    ///Loads Rom
    fn load_rom(&mut self, rom_loc: &str) {
        let mut rom_file = File::open(rom_loc).expect("[Failed] opening Rom");
        let mut rom: Vec<u8> = Vec::new();
        rom_file
            .read_to_end(&mut rom)
            .expect("[Error] Converting rom-file to u8");
        for (i, instruction) in rom.iter().enumerate() {
            self.memory.set(
                (i + cfg::MEM_START as usize).try_into().unwrap(),
                *instruction,
            );
        }
    }

    ///Opcode Exec
    fn exec(&mut self, opcode: u16) {
        match opcode {
            //CLS
            0x00E0 => self.display.clear_screen(),
            //RET
            0x00EE => self.cpu.pc = self.cpu.pop(),
            _ => self.exec_extended(opcode),
        }
    }
    ///Opcode extended
    fn exec_extended(&mut self, opcode: u16) {
        let nnn: u16 = opcode & 0x0FFF;
        let n: u8 = (opcode & 0x0F).try_into().unwrap();
        let x: u8 = ((opcode >> 8) & 0x0F).try_into().unwrap();
        let y: u8 = ((opcode >> 4) & 0x0F).try_into().unwrap();
        let kk: u8 = (opcode & 0x00FF).try_into().unwrap();

        match opcode & 0xF000 {
            //JP addr
            0x1000 => {
                self.cpu.pc = nnn;
            }
            //CALL addr
            0x2000 => {
                self.cpu.push(self.cpu.pc);
                self.cpu.pc = nnn;
            }
            //SE Vx, byte
            0x3000 => {
                if self.cpu.v[x as usize] == kk {
                    self.cpu.pc += 2;
                }
            }
            //SNE Vx, byte
            0x4000 => {
                if self.cpu.v[x as usize] != kk {
                    self.cpu.pc += 2;
                }
            }
            //SE Vx, Vy
            0x5000 => {
                if self.cpu.v[x as usize] == self.cpu.v[y as usize] {
                    self.cpu.pc += 2;
                }
            }
            //LD Vx, byte
            0x6000 => {
                self.cpu.v[x as usize] = kk;
            }
            //ADD Vx, byte
            0x7000 => {
                self.cpu.v[x as usize] = self.cpu.v[x as usize].wrapping_add(kk);
            }
            0x8000 => {
                self.extended_eight(opcode);
            }
            //SNE Vx, Vy
            0x9000 => {
                if self.cpu.v[x as usize] != self.cpu.v[y as usize] {
                    self.cpu.pc += 2;
                }
            }
            //LD I, addr
            0xA000 => {
                self.cpu.i = nnn;
            }
            //JP V0, addr
            0xB000 => {
                self.cpu.pc = self.cpu.v[0] as u16 + nnn as u16;
            }
            //RND Vx, byte
            0xC000 => {
                self.cpu.v[x as usize] = kk & rand::thread_rng().gen_range(0..0xFF);
            }
            //DRW Vx, Vy, nibble
            0xD000 => {
                self.cpu.v[0x0F] = self.display.draw_pixels(
                    self.memory.get_splice(self.cpu.i, n),
                    self.cpu.v[x as usize].into(),
                    self.cpu.v[y as usize].into(),
                ) as u8;
            }
            0xE000 => match opcode & 0x00FF {
                //SKP Vx
                0x9E => {
                    if self.keyboard.is_pressed(self.cpu.v[x as usize].into()) {
                        self.cpu.pc += 2;
                    }
                }
                //SKNP Vx
                0xA1 => {
                    if !self.keyboard.is_pressed(self.cpu.v[x as usize].into()) {
                        self.cpu.pc += 2;
                    }
                }
                _ => {}
            },
            0xF000 => {
                self.extended_f(opcode);
            }
            _ => {}
        }
    }

    #[doc(hidden)]
    fn extended_f(&mut self, opcode: u16) {
        let x: u8 = ((opcode >> 8) & 0x0F).try_into().unwrap();
        match opcode & 0xFF {
            //LD Vx, DT
            0x07 => {
                self.cpu.v[x as usize] = self.cpu.dt;
            }
            //LD Vx, K
            0x0A => {
                let key = self.keyboard.pressed_key();
                if key != 999 {
                    self.cpu.v[x as usize] = key.try_into().unwrap();
                    return;
                }
                self.cpu.pc -= 2;
            }
            //LD DT, Vx
            0x15 => {
                self.cpu.dt = self.cpu.v[x as usize];
            }
            //LD ST, Vx
            0x18 => {
                self.cpu.st = self.cpu.v[x as usize] * 10;
            }
            //ADD I, Vx
            0x1E => {
                self.cpu.i += self.cpu.v[x as usize] as u16;
            }
            //LD F, Vx (Sets the location to the sprite. For example, the 4th sprite is in loc 20 and if v[f] = 4, i = 4 * 5)
            0x29 => {
                self.cpu.i = self.cpu.v[x as usize] as u16 * 5;
            }
            //LD B, VX
            0x33 => {
                let val = self.cpu.v[x as usize];
                let h = val / 100;
                let o = val % 10;
                let t = val / 10 % 10;
                self.memory.set(self.cpu.i, h);
                self.memory.set(self.cpu.i + 1, t);
                self.memory.set(self.cpu.i + 2, o);
            }
            // LD [I], Vx
            0x55 => {
                for i in 0..=x as u16 {
                    self.memory.set(self.cpu.i + i, self.cpu.v[i as usize]);
                }
            }
            // LD Vx, [I]
            0x65 => {
                for i in 0..=x as u16 {
                    self.cpu.v[i as usize] = self.memory.get(self.cpu.i + i);
                }
            }
            _ => {}
        }
    }
    #[doc(hidden)]
    fn extended_eight(&mut self, opcode: u16) {
        let x: u8 = ((opcode >> 8) & 0x0F).try_into().unwrap();
        let y: u8 = ((opcode >> 4) & 0x0F).try_into().unwrap();
        match opcode & 0x0F {
            //LD Vx, Vy
            0x00 => {
                self.cpu.v[x as usize] = self.cpu.v[y as usize];
            }
            //OR Vx, Vy
            0x01 => {
                self.cpu.v[x as usize] |= self.cpu.v[y as usize];
            }
            //AND Vx, Vy
            0x02 => {
                self.cpu.v[x as usize] &= self.cpu.v[y as usize];
            }
            //XOR Vx, Vy
            0x03 => {
                self.cpu.v[x as usize] ^= self.cpu.v[y as usize];
            }
            //ADD Vx, Vy
            0x04 => {
                self.cpu.v[0x0F] = 0;
                let temp = self.cpu.v[x as usize] as u16 + self.cpu.v[y as usize] as u16;
                if temp > 0xFF {
                    self.cpu.v[0x0F] = 1;
                }
                self.cpu.v[x as usize] = (temp & 0xFF).try_into().unwrap();
            }
            //SUB Vx, Vy
            0x05 => {
                self.cpu.v[0x0F] = (self.cpu.v[x as usize] > self.cpu.v[y as usize]) as u8;
                self.cpu.v[x as usize] =
                    self.cpu.v[x as usize].wrapping_sub(self.cpu.v[y as usize]);
            }
            //SHR Vx {, Vy}
            0x06 => {
                self.cpu.v[0x0F] = (self.cpu.v[x as usize] & 0x01 == 1) as u8;
                self.cpu.v[x as usize] = self.cpu.v[x as usize].wrapping_div(2);
            }
            //SUBN Vx, Vy
            0x07 => {
                self.cpu.v[0x0F] = (self.cpu.v[y as usize] > self.cpu.v[x as usize]) as u8;
                self.cpu.v[x as usize] =
                    self.cpu.v[y as usize].wrapping_sub(self.cpu.v[x as usize]);
            }
            //SHL Vx {, Vy}
            0x0E => {
                self.cpu.v[0x0F] = (self.cpu.v[x as usize] >> 7 == 1) as u8;
                self.cpu.v[x as usize] = self.cpu.v[x as usize].wrapping_mul(2);
            }
            _ => {}
        }
    }
}

///Creates shader and append it to ShaderList
fn create_shader_list(shader_list: &mut Vec<Shader>) {
    let mut shader = Shader::new();
    shader.create_shader_from_file(cfg::VERTEX_LOC, cfg::FRAGMENT_LOC);
    shader_list.push(shader);
}

///Creates mesh and appends it to mesh List
fn create_objects(mesh_list: &mut Vec<Mesh>, x: u8, y: u8) {
    let mut mesh = Mesh::new(x, y);
    unsafe {
        mesh.create_mesh();
    }
    mesh_list.push(mesh);
}

///Loads the configuration file
fn load_config(config_loc: &str) -> Config {
    let config_ini = match Ini::load_from_file(config_loc) {
        Ok(val) => val,
        Err(_) => {
            let mut ini = Ini::new();
            ini.with_section(Some("Theme"))
                .set("bg", "3")
                .set("fg", "4");

            ini.with_section(Some("Hack"))
                .set("delay", "100")
                .set("cpu", "700");

            ini.with_section(Some("Screen")).set("scale", "10");

            ini.with_section(Some("Audio")).set("enable", "false");

            ini.with_section(Some("Shader"))
                .set("v_shader", "config/shader/triangle.vert")
                .set("f_shader", "config/shader/triangle.frag");

            ini.write_to_file(config_loc)
                .expect("[Failed] Creating Config.\nAborting...");

            ini
        }
    };

    let (v_shader, f_shader) = match config_ini.section(Some("Shader")) {
        Some(val) => (
            val.get("v_shader").unwrap_or(cfg::VERTEX_LOC),
            val.get("f_shader").unwrap_or(cfg::FRAGMENT_LOC),
        ),
        None => (cfg::VERTEX_LOC, cfg::FRAGMENT_LOC),
    };

    //scale
    let scale = match config_ini.section(Some("Screen")) {
        Some(val) => val.get("scale").unwrap_or("10"),
        None => "10",
    };

    let scale: u32 = scale.trim().parse().unwrap();
    //theme
    let (bg, fg) = match config_ini.section(Some("Theme")) {
        Some(val) => (val.get("bg").unwrap_or("3"), val.get("fg").unwrap_or("4")),
        None => ("3", "4"),
    };

    let bg = &cfg::PALETTE[bg.trim().parse::<usize>().unwrap()];
    let fg = &cfg::PALETTE[fg.trim().parse::<usize>().unwrap()];

    //delay
    let delay = match config_ini.section(Some("Hack")) {
        Some(val) => val.get("delay").unwrap_or("100"),
        None => "100",
    };

    let delay: u16 = delay.trim().parse().unwrap();

    let audio = match config_ini.section(Some("Audio")) {
        Some(val) => val.get("enable").unwrap_or("false"),
        None => "false",
    };

    let audio: bool = audio.trim().parse().unwrap();

    let cpu = match config_ini.section(Some("Hack")) {
        Some(val) => val.get("cpu").unwrap_or("700"),
        None => "700",
    };

    let cpu: u16 = cpu.trim().parse().unwrap();

    Config {
        v_shader: v_shader.to_owned(),
        f_shader: f_shader.to_owned(),
        scale,
        bg,
        fg,
        delay,
        audio,
        cpu,
    }
}

///Start of the emulator
pub fn load(rom_loc: &str, config_loc: &str) {
    let config = load_config(config_loc);

    let mut chip8 = Chip8::new();

    let mut main_window = Window::new(
        cfg::WIDTH * config.scale,
        cfg::HEIGHT * config.scale,
        cfg::TITLE,
    );
    main_window.init();

    let mut shader_list: Vec<Shader> = Vec::new();
    create_shader_list(&mut shader_list);

    let mut mesh_list: Vec<Mesh> = Vec::new();

    chip8.load_rom(rom_loc);
    let mut count: f32 = (main_window.get_ticks() as f32 / 1000_f32) + (1_f32 / config.cpu as f32);

    while main_window.process_events(&cfg::KEY_MAP, &mut chip8.keyboard) {
        unsafe {
            gl::ClearColor(config.bg.0, config.bg.1, config.bg.2, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            shader_list[0].use_program();

            for x in 0..cfg::WIDTH as usize {
                for y in 0..cfg::HEIGHT as usize {
                    if chip8.display.is_set(x, y) {
                        create_objects(
                            &mut mesh_list,
                            x.try_into().unwrap(),
                            y.try_into().unwrap(),
                        );
                    }
                }
            }

            for item in &mut mesh_list {
                item.render_mesh(
                    shader_list[0].get_uniform_model(),
                    shader_list[0].get_uniform_color(),
                    &config,
                );
            }

            mesh_list.clear();
        }

        main_window.swap_window();

        let opcode = chip8.memory.get_opcode(chip8.cpu.pc);
        chip8.cpu.pc += 2;
        chip8.exec(opcode);

        if chip8.cpu.st != 0 {
            if config.audio {
                main_window.play(chip8.cpu.st as u32);
            }
            chip8.cpu.st -= 1;
        }

        if chip8.cpu.dt != 0 {
            chip8.cpu.dt -= 1;
            thread::sleep(Duration::from_nanos(config.delay.try_into().unwrap()));
        }

        while count + 1_f32 / config.cpu as f32 > main_window.get_ticks() as f32 / 1000_f32 {}
        count = main_window.get_ticks() as f32 / 1000_f32;
    }
}

// #[cfg(test)]
// #[test]
// fn test() {
//     println!("Ace emulator");
//     load("games/INVADERS.ch8");
// }
