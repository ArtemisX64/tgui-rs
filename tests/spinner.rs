use tgui::TGui;
use tgui::AF;

#[test]
fn spinner() {
    let tgui = TGui::new();
    let ui = tgui.ui(None, AF::empty());
    let s = ui.spinner(None);
    s.set_list(&["Hello", "Hi", "What"]);
    std::thread::sleep(std::time::Duration::from_secs(5));
}
