use tgui::TGui;
use tgui::AF;

#[test]
fn edit_text() {
    let tgui = TGui::new();
    let ui = tgui.ui(None, AF::empty());
    ui.default_edit_text(None, "");
    std::thread::sleep(std::time::Duration::from_secs(5));
}
