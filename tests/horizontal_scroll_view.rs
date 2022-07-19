use tgui::TGui;
use tgui::AF;

#[test]
fn horizontal_scroll_view() {
    let tgui = TGui::new();
    let ui = tgui.ui(None, AF::empty());
   let h =  ui.default_nested_scroll_view(None);
   let h = ui.linear_layout(Some(&h), true);
   ui.edit_text(Some(&h), "Hello", false, true,false, "text");
    ui.button(Some(&h), "Hey");
    std::thread::sleep(std::time::Duration::from_secs(5));
}
