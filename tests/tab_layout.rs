use tgui::TGui;
use tgui::AF;

#[test]
fn tab_layout() {
    let tgui = TGui::new();
    let ui = tgui.ui(None, AF::empty());
    let tabs = ui.tab_layout(None);
    tabs.set_list(&["hello", "hi"]);
    std::thread::sleep(std::time::Duration::from_secs(5));
}
