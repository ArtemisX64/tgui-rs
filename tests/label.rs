use tgui::TGui;
use tgui::AF;

#[test]
fn label() {
    let tgui = TGui::new();
    let ui = tgui.ui(None, AF::empty());
    ui.label("Hello", None, false, false);
    std::thread::sleep(std::time::Duration::from_secs(5));
}
