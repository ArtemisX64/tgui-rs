use super::{construct_message, send_recv_msg, RawFd, View, ViewGroup};
use serde_json::json;

pub struct RadioGroup<'a> {
    aid: &'a str,
    id: i32,
    sock: &'a RawFd,
}

impl<'a> RadioGroup<'a> {
    pub fn new(fd: &'a RawFd, aid: &'a str, parent: Option<i32>) -> Self {
        let mut args = json!({ "aid": aid });

        if let Some(id) = parent {
            args["parent"] = json!(id);
        }
        let ret = send_recv_msg(fd, construct_message("createRadioGroup", &args));
        let id = ret.to_string().parse().unwrap();
        RadioGroup { id, aid, sock: fd }
    }
}

impl<'a> View for RadioGroup<'a> {
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

impl<'a> ViewGroup for RadioGroup<'a> {}
