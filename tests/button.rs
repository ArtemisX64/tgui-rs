use tgui::utils::Color;
use tgui::widgets::View;
use tgui::TGui;
use tgui::AF;

#[test]
fn button() {
    let tgui = TGui::new();
    let ui = tgui.ui(None, AF::empty());
    ui.button(None, "Hello");
    std::thread::sleep(std::time::Duration::from_secs(5));
}

#[test]
fn change_color() {
    let tgui = TGui::new();
    let ui = tgui.ui(None, AF::empty());
    let button = ui.button(None, "Hey");
    button.set_background_color(Color::from_rgb(198, 120, 236));

    std::thread::sleep(std::time::Duration::from_secs(5));
}
