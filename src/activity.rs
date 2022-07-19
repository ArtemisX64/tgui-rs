use super::connection::{construct_message, send_msg, send_recv_msg};
use super::utils::{Color, Vec2};
use super::{RawFd, AF};
use serde_json::json;
use std::io::Cursor;

pub struct Activity {
    pub tid: i32,
    pub aid: String,
}

impl Activity {
    pub fn new(main: &RawFd, tid: Option<i32>, flags: AF) -> Self {
        let mut args = json!({
            "dialog": flags.contains(AF::DIALOG),
            "pip": flags.contains(AF::PIP),
            "lockscreen": flags.contains(AF::LOCK_SCREEN),
            "canceloutside": flags.contains(AF::CANCEL_OUTSIDE),
            "overlay": flags.contains(AF::OVERLAY)
        });
        let mut new_tid = 0i32;
        if let Some(val) = tid {
            args["tid"] = json!(val);
            new_tid = val;
        }

        let ret = send_recv_msg(main, construct_message("newActivity", &args));

        let aid: Vec<u8> = ret[0]
            .to_string()
            .as_bytes()
            .iter()
            .map(|&val| val)
            .filter(|&val| val != b'\"')
            .collect();
        let aid = String::from_utf8(aid).unwrap();

        if let None = tid {
            new_tid = ret[1].to_string().parse().unwrap();
        }
        Activity { tid: new_tid, aid }
    }

    pub fn finish(&self, main: &RawFd) {
        let args = json!({
            "aid": &self.aid
        });
        send_msg(main, construct_message("finishActivity", &args));
    }

    pub fn set_input_mode(&self, main: &RawFd, mode: &str) {
        let args = json!({
            "aid": &self.aid,
            "mode": mode
        });

        send_msg(main, construct_message("setInputMode", &args));
    }

    pub fn set_pip_mode(&self, main: &RawFd, pip: bool) {
        let args = json!({"aid": &self.aid, "pip": pip});
        send_msg(main, construct_message("setPiPMode", &args));
    }

    pub fn set_pip_mode_auto(&self, main: &RawFd, pip: bool) {
        let args = json!({"aid": &self.aid, "pip": pip});
        send_msg(main, construct_message("setPiPModeAuto", &args));
    }

    pub fn set_pip_params(&self, main: &RawFd, aspect_ratio: Vec2<u16>) {
        let args = json!({"aid": &self.aid, "num": aspect_ratio.x, "den": aspect_ratio.y});
        send_msg(main, construct_message("setPiPParams", &args));
    }

    pub fn keep_screen_on(&self, main: &RawFd, on: bool) {
        let args = json!({"aid": &self.aid, "on": on});
        send_msg(main, construct_message("keepScreenOn", &args));
    }

    pub fn set_task_description(&self, main: &RawFd, img: &str, label: &str) {
        let base_img = image::open(img).unwrap();
        let mut buff: Vec<u8> = Vec::new();
        let res_base64 =
            match base_img.write_to(&mut Cursor::new(&mut buff), image::ImageOutputFormat::Png) {
                Ok(_) => base64::encode(&buff),
                Err(_) => base64::encode(&[0u8]),
            };

        let args = json!({
            "aid": &self.aid,
            "img": res_base64,
            "label": label,
        });
        send_msg(main, construct_message("setTaskDescription", &args));
    }

    pub fn set_theme(
        &self,
        main: &RawFd,
        status_bar_color: Color,
        color_primary: Color,
        window_background: Color,
        text_color: Color,
        color_accent: Color,
    ) {
        let args = json!({
            "aid": &self.aid,
            "statusBarColor": status_bar_color.to_u32(),
            "colorPrimary": color_primary.to_u32(),
            "windowBackground": window_background.to_u32(),
            "textColor": text_color.to_u32(),
            "colorAccent": color_accent.to_u32()
        });
        send_msg(main, construct_message("setTheme", &args));
    }

    pub fn set_orientation(&self, main: &RawFd, orientation: &str) {
        let args = json!({"aid": &self.aid, "orientation": orientation});
        send_msg(main, construct_message("setOrientation", &args));
    }

    pub fn set_position(&self, main: &RawFd, position: Vec2<u16>) {
        let args = json!({"aid": &self.aid, "x": position.x, "y": position.y});
        send_msg(main, construct_message("setPosition", &args));
    }

    pub fn hide_soft_keyboard(&self, main: &RawFd) {
        let args = json!({"aid": &self.aid});
        send_msg(main, construct_message("hideSoftKeyboard", &args));
    }

    pub fn request_unlock(&self, main: &RawFd) {
        let args = json!({"aid": &self.aid});
        send_msg(main, construct_message("requestUnlock", &args));
    }

    pub fn move_to_back(&self, main: &RawFd) {
        let args = json!({"aid": &self.aid});
        send_msg(main, construct_message("moveTaskToBack", &args));
    }

    pub fn get_configuration(&self, main: &RawFd) -> serde_json::Value {
        let args = json!({"aid": &self.aid});
        send_recv_msg(main, construct_message("getConfiguration", &args))
    }

    pub fn send_overlay_events(&self, main: &RawFd, send: bool) {
        let args = json!({"aid": &self.aid, "send": send});
        send_msg(main, construct_message("sendOverlayTouchEvents", &args));
    }
}
