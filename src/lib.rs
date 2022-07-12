use serde_json::json;
use std::os::unix::io::RawFd;

mod connection;

pub mod activity;
pub mod event;
pub mod layouts;
pub mod ui;
pub mod utils;
pub mod widgets;

use widgets::View;

bitflags::bitflags! {
    pub struct AF: u8{
        const DIALOG = 0b0000_0001;
        const PIP = 0b0000_0010;
        const CANCEL_OUTSIDE = 0b0000_0100;
        const LOCK_SCREEN = 0b0000_1000;
        const OVERLAY = 0b0001_0000;
    }
}

pub struct TGui {
    pub main: RawFd,
    pub event: RawFd,
}

impl TGui {
    pub fn new() -> Self {
        let (main, event) = connection::connect();
        TGui { main, event }
    }

    pub fn activity(&self, tid: Option<i32>, flags: AF) -> activity::Activity {
        activity::Activity::new(self.main, tid, flags)
    }

    pub fn ui(&self, tid: Option<i32>, flags: AF) -> ui::Ui {
        ui::Ui::new(self.main, tid, flags)
    }

    pub fn event(&self) -> event::Event {
        let event = connection::recv_msg(self.event);
        event::Event::new(event)
    }

    pub fn toast(&self, text: &str, long: bool) {
        let args = json!({
            "text": text,
            "long": long
        });
        connection::send_msg(self.main, connection::construct_message("toast", &args));
    }

    pub fn turn_screen_on(&self) {
        connection::send_msg(
            self.main,
            connection::construct_message("turnScreenOn", &json!(null)),
        );
    }

    pub fn is_locked(&self) -> bool {
        connection::send_recv_msg(
            self.main,
            connection::construct_message("isLocked", &json!(null)),
        )
        .to_string()
        .parse()
        .unwrap()
    }
}
