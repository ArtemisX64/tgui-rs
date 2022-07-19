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

use glfw::{Action, Context, Key};
use std::ops::Drop;
use std::sync::mpsc::Receiver;

///Window Struct
pub struct Window {
    glfw: glfw::Glfw,
    window: Option<glfw::Window>,
    events: Option<Receiver<(f64, glfw::WindowEvent)>>,
    width: u32,
    height: u32,
    title: &'static str,
}
///Window Module
impl Window {
    ///Creates a Window struct with width, height & title
    pub fn new(width: u32, height: u32, title: &'static str) -> Self {
        Window {
            glfw: glfw::init(glfw::FAIL_ON_ERRORS)
                .expect("[Error] Initialising GLFW.\nAborting..."),
            window: None,
            events: None,
            width,
            height,
            title,
        }
    }

    ///It creates window and events\
    ///Also, enables key polling and creates an opengl context  
    pub fn init(&mut self) {
        self.glfw
            .window_hint(glfw::WindowHint::ContextVersion(3, 3));
        self.glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));
        self.glfw
            .window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

        let (mut window, events) = self
            .glfw
            .create_window(
                self.width,
                self.height,
                self.title,
                glfw::WindowMode::Windowed,
            )
            .expect("[Error] Creating Window\nAborting...");

        window.make_current();
        self.glfw.set_swap_interval(glfw::SwapInterval::Sync(0));
        //window.set_key_polling(true);
        //window.set_framebuffer_size_polling(true);
        window.set_all_polling(true);
        window.set_resizable(false);

        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
        let (buffersize_width, buffersize_height) = window.get_framebuffer_size();
        unsafe {
            gl::Viewport(0, 0, buffersize_width, buffersize_height);
        }

        self.window = Some(window);
        self.events = Some(events);
    }

    ///Checks if window should close
    pub fn should_close(&self) -> bool {
        self.window.as_ref().unwrap().should_close()
    }

    ///Poll Events
    pub fn poll_events(&mut self) {
        self.glfw.poll_events();
    }

    ///Swap Buffers
    pub fn swap_buffers(&mut self) {
        self.window.as_mut().unwrap().swap_buffers();
    }

    ///Handles Input events
    pub fn process_events(&mut self, key_map: &[Key], keyboard: &mut crate::Keyboard) {
        for (_, event) in glfw::flush_messages(self.events.as_ref().unwrap()) {
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    self.window.as_mut().unwrap().set_should_close(true)
                }
                glfw::WindowEvent::Key(key, _, Action::Press, _) => keyboard.on_press(key_map, key),
                glfw::WindowEvent::Key(key, _, Action::Release, _) => {
                    keyboard.on_release(key_map, key)
                }
                _ => {}
            }
        }
    }
}
