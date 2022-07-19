use super::RawFd;
use super::{construct_message, send_recv_msg, Color, View};
use serde_json::json;

pub struct Label<'a> {
    id: i32,
    aid: &'a str,
    sock: &'a RawFd,
}

impl<'a> Label<'a> {
    pub fn new(
        fd: &'a RawFd,
        aid: &'a str,
        parent: Option<i32>,
        text: &str,
        selectable_text: bool,
        clickable_links: bool,
    ) -> Self {
        let mut args = json!({
            "aid": aid,
            "text": text,
            "selectableText": selectable_text,
            "clickableLinks": clickable_links
        });

        if let Some(id) = parent {
            args["parent"] = json!(id);
        }

        let ret = send_recv_msg(fd, construct_message("createTextView", &args));
        let id: i32 = ret.to_string().parse().unwrap();
        Label { id, aid, sock: fd }
    }
}

pub trait TextView: View {
    fn set_text_size(&self, size: u8) {
        let args = json!({
            "aid": self.get_aid(),
            "id": self.get_id(),
            "size": size
        });

        self.send_msg(construct_message("setTextSize", &args));
    }

    fn set_text(&self, text: &str) {
        let args = json!({
            "aid": self.get_aid(),
            "id": self.get_id(),
            "text": text
        });

        self.send_msg(construct_message("setText", &args));
    }

    fn get_text(&self) -> String {
        let args = json!({
            "aid": self.get_aid(),
            "id": self.get_id()
        });

        let ret = self.send_recv_msg(construct_message("getText", &args)).to_string();
        let ret = ret.as_bytes();
        let ret: Vec<u8> = ret.iter().map(|&val| {val}).filter(|&val| {val != b'\"'}).collect();
        String::from_utf8(ret).unwrap()
    }

    fn set_text_color(&self, color: Color) {
        let args = json!({
            "aid": self.get_aid(),
            "id": self.get_id(),
            "color": color.to_u32()
        });

        self.send_msg(construct_message("setTextColor", &args));
    }

    fn set_text_event(&self, send: bool) {
        let args = json!({
            "aid": self.get_aid(),
            "id": self.get_id(),
            "send": send
        });

        self.send_msg(construct_message("setTextEvent", &args));
    }
}

impl<'a> TextView for Label<'a> {}

impl<'a> View for Label<'a> {
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
