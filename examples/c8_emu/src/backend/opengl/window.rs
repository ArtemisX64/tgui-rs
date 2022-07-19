//!# Window
//!This module creates a glfw context and it's window.\
//!Also, this module is responsible for handling inputs.\
//!## Example
//! ```
//!
//! use opengl::window::Window;
//! let mut main_window =  Window::new(width, height, title);
//!
//! main_window.init();
//! while !window.should_close(){
//! window.process_events();
//! }
//! ```

use sdl2::audio::{AudioCallback, AudioSpecDesired};
use sdl2::event::Event;
use sdl2::keyboard::Keycode as KeyCode;
use std::convert::TryInto;




#[doc(hidden)]
struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32,
}

#[doc(hidden)]
impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        // Generate a square wave
        for x in out.iter_mut() {
            *x = if self.phase <= 0.5 {
                self.volume
            } else {
                -self.volume
            };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}

///Window Struct
pub struct Window {
    pub sdl: sdl2::Sdl,
    window: Option<sdl2::video::Window>,
    _gl_context: Option<sdl2::video::GLContext>,
    event_pump: Option<sdl2::EventPump>,
    width: u32,
    height: u32,
    title: &'static str,
    device: Option<sdl2::audio::AudioDevice<SquareWave>>,
}

///Window Module
impl Window {
    pub fn new(width: u32, height: u32, title: &'static str) -> Self {
        Window {
            sdl: sdl2::init().expect("[Error] Initialising SDL.\nAborting..."),
            window: None,
            _gl_context: None,
            event_pump: None,
            width,
            height,
            title,
            device: None,
        }
    }

    ///It creates window and events\
    ///Also, enables key polling and creates an opengl context  
    pub fn init(&mut self) {
        let video_subsystem = self.sdl.video().unwrap();
        let gl_attr = video_subsystem.gl_attr();

        gl_attr.set_context_version(3, 3);
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_flags().forward_compatible();

        let window = video_subsystem
            .window(self.title, self.width, self.height)
            .opengl()
            .position_centered()
            .build()
            .unwrap();

        let gl_context = window.gl_create_context().unwrap();
        video_subsystem.gl_set_swap_interval(0).unwrap();
        gl::load_with(|symbol| video_subsystem.gl_get_proc_address(symbol) as *const _);

        let (buffersize_width, buffersize_height) = window.drawable_size();
        unsafe {
            gl::Viewport(
                0,
                0,
                buffersize_width.try_into().unwrap(),
                buffersize_height.try_into().unwrap(),
            );
        }

        let event_pump = self.sdl.event_pump().unwrap();

        self.window = Some(window);
        self._gl_context = Some(gl_context);
        self.event_pump = Some(event_pump);

        let audio_subsystem = self.sdl.audio().unwrap();
        let desired_spec = AudioSpecDesired {
            freq: Some(44100),
            channels: Some(1),
            samples: None,
        };

        let device = audio_subsystem
            .open_playback(None, &desired_spec, |spec| SquareWave {
                phase_inc: 440.0 / spec.freq as f32,
                phase: 0.0,
                volume: 0.25,
            })
            .unwrap();
        self.device = Some(device);
    }

    ///Get frame time
    pub fn get_ticks(&self) -> u32 {
        self.sdl.timer().unwrap().ticks()
    }

    ///Swap Window
    pub fn swap_window(&mut self) {
        self.window.as_mut().unwrap().gl_swap_window();
    }

    ///Play Audio
    pub fn play(&mut self, st: u32) {
        self.device.as_mut().unwrap().resume();
        if st <= 1 {
            self.device.as_mut().unwrap().pause();
        }
    }

    ///Handles Input events and window state
    pub fn process_events(&mut self, key_map: &[KeyCode], keyboard: &mut crate::Keyboard) -> bool {
        for event in self.event_pump.as_mut().unwrap().poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(KeyCode::Escape),
                    ..
                } => return false,
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => keyboard.on_press(key_map, keycode),
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => keyboard.on_release(key_map, keycode),
                _ => {}
            }
        }
        true
    }
}
