use tgui::widgets::{label::TextView, View};
use tgui::TGui;
use tgui::AF;

fn main() {
    let tgui = TGui::new();
    let mut flags = AF::empty();
    flags.set(AF::DIALOG, true);
    flags.set(AF::CANCEL_OUTSIDE, true);

    let ui = tgui.ui(None, flags);
    let layout = ui.linear_layout(None, true);

    let title = ui.label(Some(&layout), "Download Video", false, false);
    title.set_text_size(30);

    title.set_margin(5, None);

    ui.label(Some(&layout), "Video Link", false, false);
    ui.edit_text(Some(&layout), "", false, false, false, "text");

    ui.label(Some(&layout), "File Name", false, false);
    ui.edit_text(Some(&layout), "", false, false, false, "text");

    let buttons = ui.linear_layout(Some(&layout), false);
    ui.button(Some(&buttons), "Download");
    let cancel = ui.button(Some(&buttons), "Cancel");

    loop {
        let event = tgui.event();
        if event.ty == tgui::event::DESTROY
            && event.value["finishing"].to_string().trim().parse().unwrap()
        {
            std::process::exit(0);
        }
        if event.ty == tgui::event::CLICK && event.id == cancel.get_id() {
            ui.finish();
        }
    }
}
