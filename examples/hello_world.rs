use tgui::utils::Color;
use tgui::widgets::label::TextView;
use tgui::TGui;
use tgui::AF;

fn main() {
    let tgui = TGui::new();
    let ui = tgui.ui(None, AF::empty());
    let label = ui.label("Hello", None, false, false);
    std::thread::sleep(std::time::Duration::from_secs(5));
    label.set_text("Bye World");
    label.set_text_color(Color::from_rgb(160, 200, 240));
    std::thread::sleep(std::time::Duration::from_secs(5));
}
