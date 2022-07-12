use tgui::TGui;
use tgui::AF;

#[test]
fn edit_text() {
    let tgui = TGui::new();
    let ui = tgui.ui(None, AF::empty());
    let _et = ui.edit_text("", None, false, true, false, "text");
    std::thread::sleep(std::time::Duration::from_secs(5));
}
