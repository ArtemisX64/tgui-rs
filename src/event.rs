use serde_json::{json, Value};

pub const CLICK: &str = "click";
pub const LONG_CLICK: &str = "longClick";
pub const FOCUS_CHANGE: &str = "focusChange";
pub const KEY: &str = "key";
pub const TOUCH: &str = "touch";
pub const REFRESH: &str = "refresh";
pub const SELECTED: &str = "selected";
pub const ITEM_SELECTED: &str = "itemselected";

pub const CREATE: &str = "create";
pub const START: &str = "start";
pub const RESUME: &str = "resume";
pub const PAUSE: &str = "pause";
pub const STOP: &str = "stop";
pub const DESTROY: &str = "destroy";

pub const USER_LEAVE_HINT: &str = "UserLeaveHint";
pub const PIP_CHANGED: &str = "pipchanged";
pub const CONFIG: &str = "config";

pub const SCREEN_ON: &str = "screen_on";
pub const SCREEN_OFF: &str = "screen_off";
pub const TIMEZONE: &str = "timezone";
pub const LOCALE: &str = "locale";
pub const AIRPLANE: &str = "airplane";

pub const OVERLAY_TOUCH: &str = "overlay_touch";
pub const OVERLAY_SCALE: &str = "overlay_scale";

pub const TOUCH_UP: &str = "up";
pub const TOUCH_DOWN: &str = "down";
pub const TOUCH_POINTER_UP: &str = "pointer_up";
pub const TOUCH_POINTER_DOWN: &str = "pointer_down";
pub const TOUCH_CANCEL: &str = "cancel";
pub const TOUCH_MOVE: &str = "move";

pub struct Event {
    pub ty: String,
    pub value: Value,
    pub aid: String,
    pub id: i32,
}

impl Event {
    pub fn new(ev: Value) -> Self {
        let ty_b = ev["type"].to_string();
        let ty_b: Vec<u8> = ty_b
            .as_bytes()
            .iter()
            .map(|&val| val)
            .filter(|&val| val != b'\"')
            .collect();
        let ty = String::from_utf8(ty_b).unwrap();
        let value = ev["value"].clone();
        let mut aid = String::new();
        let mut id = 0i32;
        if value["aid"] != json!(null) {
            let a_b = value["aid"].to_string();
            let a_b = a_b.as_bytes();
            let a_b: Vec<u8> = a_b
                .iter()
                .map(|&val| val)
                .filter(|&val| val != b'\"')
                .collect();
            aid = String::from_utf8(a_b).unwrap();
            id = match value["id"].to_string().parse() {
                Ok(val) => val,
                Err(_) => 0,
            };
        }
        Event { ty, value, aid, id }
    }
}
