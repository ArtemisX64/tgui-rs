use tgui::TGui;
use tgui::AF;

#[test]
fn image() {
    let tgui = TGui::new();
    let ui = tgui.ui(None, AF::empty());
    let img = ui.image_view(None);
    img.set_image("res/rust.png");
    std::thread::sleep(std::time::Duration::from_secs(5));
}
