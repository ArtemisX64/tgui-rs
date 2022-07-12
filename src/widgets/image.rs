use super::RawFd;
use super::{construct_message, send_msg, send_recv_msg, View};
use serde_json::json;
use std::io::Cursor;

pub struct ImageView<'a> {
    id: i32,
    aid: &'a str,
    sock: RawFd,
}

impl<'a> ImageView<'a> {
    pub fn new(fd: RawFd, aid: &'a str, parent: Option<i32>) -> Self {
        let mut args = json!({ "aid": aid });

        if let Some(id) = parent {
            args["parent"] = json!(id);
        }

        let ret = send_recv_msg(fd, construct_message("createImageView", &args));
        let id: i32 = ret.to_string().parse().unwrap();
        ImageView { id, aid, sock: fd }
    }

    pub fn set_image(&self, img: &str) {
        let base_img = image::open(img).unwrap();
        let mut buff: Vec<u8> = Vec::new();
        base_img
            .write_to(&mut Cursor::new(&mut buff), image::ImageOutputFormat::Png)
            .unwrap();
        let res_base64 = base64::encode(&buff);
        let args = json!({
            "aid": &self.aid,
            "id": self.id,
            "img": res_base64
        });
        send_msg(self.sock, construct_message("setImage", &args));
    }
}

impl<'a> View for ImageView<'a> {
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
