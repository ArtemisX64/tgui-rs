use super::activity::Activity;
use super::layouts::linear_layout::LinearLayout;
use super::widgets::{button::Button, edit_text::EditText, image::ImageView, label::Label, View};
use super::RawFd;
use super::AF;

pub struct Ui {
    activity: Activity,
    main: RawFd,
}

impl Ui {
    pub fn new(main: RawFd, tid: Option<i32>, flags: AF) -> Self {
        Ui {
            activity: Activity::new(main, tid, flags),
            main,
        }
    }

    pub fn label(
        &self,
        text: &str,
        parent: Option<&dyn View>,
        selectable_text: bool,
        clickable_links: bool,
    ) -> Label {
        let parent: Option<i32> = match parent {
            Some(parent) => Some(parent.get_id()),
            None => None,
        };
        Label::new(
            self.main,
            &self.activity.aid,
            text,
            parent,
            selectable_text,
            clickable_links,
        )
    }

    pub fn image_view(&self, parent: Option<&dyn View>) -> ImageView {
        let parent: Option<i32> = match parent {
            Some(parent) => Some(parent.get_id()),
            None => None,
        };
        ImageView::new(self.main, &self.activity.aid, parent)
    }
    pub fn button(&self, text: &str, parent: Option<&dyn View>) -> Button {
        let parent: Option<i32> = match parent {
            Some(parent) => Some(parent.get_id()),
            None => None,
        };
        Button::new(self.main, &self.activity.aid, text, parent)
    }

    pub fn edit_text(
        &self,
        text: &str,
        parent: Option<&dyn View>,
        single_line: bool,
        line: bool,
        block_input: bool,
        ty: &str,
    ) -> EditText {
        let parent: Option<i32> = match parent {
            Some(parent) => Some(parent.get_id()),
            None => None,
        };
        EditText::new(
            self.main,
            &self.activity.aid,
            text,
            parent,
            single_line,
            line,
            block_input,
            ty,
        )
    }
    pub fn linear_layout(&self, parent: Option<&dyn View>, vertical: bool) -> LinearLayout {
        let parent: Option<i32> = match parent {
            Some(parent) => Some(parent.get_id()),
            None => None,
        };
        LinearLayout::new(self.main, &self.activity.aid, parent, vertical)
    }

    pub fn finish(&self) {
        self.activity.finish(self.main);
    }
}
