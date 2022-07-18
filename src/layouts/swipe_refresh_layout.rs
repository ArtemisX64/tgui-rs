use super::{construct_message, send_recv_msg, RawFd, View, ViewGroup};
use serde_json::json;

pub struct SwipeRefreshLayout<'a> {
    aid: &'a str,
    id: i32,
    sock: &'a RawFd,
}

impl<'a> SwipeRefreshLayout<'a> {
    pub fn new(fd: &'a RawFd, aid: &'a str, parent: Option<i32>) -> Self {
        let mut args = json!({ "aid": aid });

        if let Some(id) = parent {
            args["parent"] = json!(id);
        }
        let ret = send_recv_msg(fd, construct_message("createSwipeRefreshLayout", &args));
        let id = ret.to_string().parse().unwrap();
        SwipeRefreshLayout { id, aid, sock: fd }
    }

    pub fn set_refreshing(&self, refresh: bool) {
        let args = json!({
            "aid": &self.aid,
            "id": &self.id,
            "refresh": refresh
        });
        self.send_msg(construct_message("setRefreshing", &args));
    }
}

impl<'a> View for SwipeRefreshLayout<'a> {
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

impl<'a> ViewGroup for SwipeRefreshLayout<'a> {}
