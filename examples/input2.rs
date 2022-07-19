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

    let title = ui.default_label(Some(&layout), "Input Demo");
    title.set_text_size(30);

    title.set_margin(5, None);

    let switch = ui.default_switch(Some(&layout), "Switch");

    loop {
        let event = tgui.event();
        if event.ty == tgui::event::DESTROY
            && event.value["finishing"].to_string().trim().parse().unwrap()
        {
            std::process::exit(0);
        }
        if event.ty == tgui::event::CLICK && event.id == switch.get_id() {
            ui.activity.finish(ui.main);
        }
    }
}
