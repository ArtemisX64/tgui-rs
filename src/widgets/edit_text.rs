use super::label::TextView;
use super::RawFd;
use super::{construct_message, send_msg, send_recv_msg, View};
use serde_json::json;

pub struct EditText<'a> {
    id: i32,
    aid: &'a str,
    sock: RawFd,
}

impl<'a> EditText<'a> {
    pub fn new(
        fd: RawFd,
        aid: &'a str,
        text: &str,
        parent: Option<i32>,
        single_line: bool,
        line: bool,
        block_input: bool,
        ty: &str,
    ) -> Self {
        let mut args = json!({
            "aid": aid,
            "text": text,
            "singleline": single_line,
            "line": line,
            "blockinput": block_input,
            "type": ty
        });

        if let Some(id) = parent {
            args["parent"] = json!(id);
        }

        let ret = send_recv_msg(fd, construct_message("createEditText", &args));
        let id: i32 = ret.to_string().parse().unwrap();
        EditText { id, aid, sock: fd }
    }

    pub fn show_cursor(&self, show: bool) {
        let args = json!({
            "aid": &self.aid,
            "id": self.id,
            "show": show

        });
        send_msg(self.sock, construct_message("showCursor", &args));
    }
}

impl<'a> TextView for EditText<'a> {}

impl<'a> View for EditText<'a> {
    fn get_id(&self) -> i32 {
        self.id
    }

    fn get_aid(&self) -> &str {
        self.aid
    }

    fn get_sock(&self) -> RawFd {
        self.sock
    }
}
