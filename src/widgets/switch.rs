use super::compound_button::CompoundButton;
use super::label::TextView;
use super::RawFd;
use super::{construct_message, send_recv_msg, View};
use serde_json::json;

pub struct Switch<'a> {
    id: i32,
    aid: &'a str,
    sock: &'a RawFd,
    check: bool,
}

impl<'a> Switch<'a> {
    pub fn new(fd: &'a RawFd, aid: &'a str, parent: Option<i32>, text: &str, check: bool) -> Self {
        let mut args = json!({
            "aid": aid,
            "text": text,
            "checked": check
        });

        if let Some(id) = parent {
            args["parent"] = json!(id);
        }

        let ret = send_recv_msg(fd, construct_message("createSwitch", &args));
        let id: i32 = ret.to_string().parse().unwrap();
        Switch {
            id,
            aid,
            sock: fd,
            check,
        }
    }
}

impl<'a> TextView for Switch<'a> {}

impl<'a> CompoundButton for Switch<'a> {
    fn check(&mut self, set: bool) {
        self.check = set;
    }
}

impl<'a> View for Switch<'a> {
    fn get_id(&self) -> i32 {
        self.id
    }

    fn get_aid(&self) -> &str {
        self.aid
    }

    fn get_sock(&self) -> &RawFd {
        self.sock
    }
}
