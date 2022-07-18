use super::connection::{construct_message, send_recv_msg};
use super::utils::Vec2;
use super::{RawFd, View};
use serde_json::json;

pub mod frame_layout;
pub mod horizontal_scroll_view;
pub mod linear_layout;
pub mod nested_scroll_view;
pub mod radio_group;
pub mod swipe_refresh_layout;
pub mod tab_layout;

pub trait ViewGroup: View {
    fn clear_children(&self) {
        let args = json!({
            "aid": self.get_aid(),
            "id": self.get_id()
        });

        self.send_msg(construct_message("deleteChildren", &args));
    }
}
