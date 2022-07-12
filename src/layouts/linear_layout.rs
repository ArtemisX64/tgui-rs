use super::{construct_message, send_recv_msg, RawFd, View, ViewGroup};
use serde_json::json;

pub struct LinearLayout<'a> {
    aid: &'a str,
    id: i32,
    sock: RawFd,
}

impl<'a> LinearLayout<'a> {
    pub fn new(fd: RawFd, aid: &'a str, parent: Option<i32>, vertical: bool) -> Self {
        let mut args = json!({
            "aid": aid,
            "vertical": vertical
        });

        if let Some(id) = parent {
            args["parent"] = json!(id);
        }
        let ret = send_recv_msg(fd, construct_message("createLinearLayout", &args));
        let id = ret.to_string().parse().unwrap();
        LinearLayout { id, aid, sock: fd }
    }
}

impl<'a> View for LinearLayout<'a> {
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

impl<'a> ViewGroup for LinearLayout<'a> {}
