use tgui::TGui;
use tgui::ui::View;
use tgui::AF;
use tgui::utils::{Vec2, Color};

#[test]
fn space() {
    let tgui = TGui::new();
    let ui = tgui.ui(None, AF::empty());
    let rect = ui.space(None);
    rect.set_background_color(Color::from_rgb(127, 178, 230));
    rect.set_dimensions(Vec2{x: 100, y: 100},true);
    std::thread::sleep(std::time::Duration::from_secs(5));
}
