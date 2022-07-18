use super::RawFd;
use super::{construct_message, send_recv_msg, View};
use serde_json::json;

pub struct ProgressBar<'a> {
    id: i32,
    aid: &'a str,
    sock: &'a RawFd,
}

impl<'a> ProgressBar<'a> {
    pub fn new(fd: &'a RawFd, aid: &'a str, parent: Option<i32>) -> Self {
        let mut args = json!({
            "aid": aid,
        });

        if let Some(id) = parent {
            args["parent"] = json!(id);
        }

        let ret = send_recv_msg(fd, construct_message("createProgressBar", &args));
        let id: i32 = ret.to_string().parse().unwrap();
        ProgressBar { id, aid, sock: fd }
    }

    pub fn set_progress(&self, progress: u8) {
        let args = json!({"aid": &self.aid, "id": &self.id, "progress": progress});
        self.send_msg(construct_message("createProgressBar", &args));
    }
}

impl<'a> View for ProgressBar<'a> {
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
