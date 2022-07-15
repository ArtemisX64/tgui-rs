use super::connection::{construct_message, send_msg, send_recv_msg};
use super::{RawFd, AF};
use serde_json::json;

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
}
