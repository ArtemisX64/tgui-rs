use super::Vec2;
use super::{construct_message, send_recv_msg, RawFd, View, ViewGroup};
use serde_json::json;

pub struct NestedScrollView<'a> {
    aid: &'a str,
    id: i32,
    sock: &'a RawFd,
}

impl<'a> NestedScrollView<'a> {
    pub fn new(
        fd: &'a RawFd,
        aid: &'a str,
        parent: Option<i32>,
        fill_viewport: bool,
        snapping: bool,
        no_bar: bool,
    ) -> Self {
        let mut args = json!({ "aid": aid, "fillviewport": fill_viewport, "snapping": snapping, "nobar": no_bar});

        if let Some(id) = parent {
            args["parent"] = json!(id);
        }
        let ret = send_recv_msg(fd, construct_message("createNestedScrollView", &args));
        let id = ret.to_string().parse().unwrap();
        NestedScrollView { id, aid, sock: fd }
    }

    pub fn set_scroll_position(&self, pos: Vec2<u16>, smooth: bool) {
        let args = json!({
           "aid": &self.aid,
           "id": &self.id,
           "x": pos.x,
           "y": pos.y,
           "soft": smooth
        });
        self.send_msg(construct_message("setScrollPosition", &args));
    }

    pub fn get_scroll_position(&self) -> Vec2<u16> {
        let args = json!({
           "aid": &self.aid,
           "id": &self.id
        });
        let ret = self.send_recv_msg(construct_message("getScrollPosition", &args));
        let x: u16 = ret["x"].to_string().parse().unwrap();
        let y: u16 = ret["y"].to_string().parse().unwrap();
        Vec2 { x, y }
    }
}

impl<'a> View for NestedScrollView<'a> {
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

impl<'a> ViewGroup for NestedScrollView<'a> {}
