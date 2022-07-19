//!# TGui Frontend
//!It uses TermuxGUI in android
//!```
//! ./c8_tgui
//!```
//!

use ini::Ini;
use tgui::utils::{Color};
use tgui::{
    ui::{TextView, View},
    TGui, AF,
};

///The start of c8_tgui
fn main() {
    let _config = Ini::new();
    start_page();
}

fn start_page() {
    let tgui = TGui::new();
    let mut flags = AF::empty();
    flags.set(AF::DIALOG, true);
    flags.set(AF::CANCEL_OUTSIDE, true);

    let ui = tgui.ui(None, flags);
    let linear_layout = ui.linear_layout(None, true);

    let title = ui.default_label(Some(&linear_layout), "Chip 8 emulator");
    title.set_text_size(30);
    title.set_margin(5, None);

    let rom_layout = ui.linear_layout(Some(&linear_layout), false);
    let rom_header = ui.default_label(Some(&rom_layout), "Rom:");
    rom_header.set_text_size(18);
    rom_header.set_margin(5, None);
    let rom = ui.default_edit_text(Some(&rom_layout), "");

    let settings = ui.button(Some(&linear_layout), "Settings");
    let button_layout = ui.linear_layout(Some(&linear_layout), false);
    let start = ui.button(Some(&button_layout), "Start");
    start.set_background_color(Color::from_rgb(0, 0, 255));

    let exit = ui.button(Some(&button_layout), "Exit");
    exit.set_background_color(Color::from_rgb(255, 0, 0));

    loop {
        let event = tgui.event();
        if event.ty == tgui::event::DESTROY
            && event.value["finishing"].to_string().trim().parse().unwrap()
        {
            break;
        }
        if event.ty == tgui::event::CLICK {
            if event.id == exit.get_id() {
                ui.activity.finish(&ui.main);
            } else if event.id == settings.get_id() {
                settings_page(&tgui);
            } else if event.id == start.get_id() {
                let rom = rom.get_text();
                chip8::load(&rom, "config/config.ini");

            }
        }
    }
}

fn settings_page(tgui: &TGui) {
    let audio: bool;
    let mut cpu: String;
    let mut delay: String;
    let mut scale: String;
    let mut config = match Ini::load_from_file("config/config.ini") {
        Ok(val) => {
            audio = val
                .section(Some("Audio"))
                .unwrap()
                .get("enable")
                .unwrap_or("false")
                .parse()
                .unwrap();
            cpu = val
                .section(Some("Hack"))
                .unwrap()
                .get("cpu")
                .unwrap_or("10")
                .to_owned();
            delay = val
                .section(Some("Hack"))
                .unwrap()
                .get("delay")
                .unwrap_or("100")
                .to_owned();
             scale = val
                .section(Some("Screen"))
                .unwrap()
                .get("scale")
                .unwrap_or("10")
                .to_owned();

            val
        }
        Err(_) => todo!(),
    };
    let mut flags = AF::empty();
    flags.set(AF::DIALOG, true);
    flags.set(AF::CANCEL_OUTSIDE, true);

    let settings_ui = tgui.ui(None, flags);
    let layout = settings_ui.linear_layout(None, true);
    let title = settings_ui.default_label(Some(&layout), "Settings");
    title.set_text_size(30);
    title.set_margin(5, None);

    let nested_scroll_view = settings_ui.nested_scroll_view(Some(&layout), true, false, false);
    let settings_layout = settings_ui.linear_layout(Some(&nested_scroll_view), true);

    let audio_button = settings_ui.switch(Some(&settings_layout), "Audio", audio);
    audio_button.set_text_size(12);

    settings_ui.default_label(Some(&settings_layout), "Background Color");
    let bg_spinner = settings_ui.spinner(Some(&settings_layout));
    bg_spinner.set_list(&["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"]);

    settings_ui.default_label(Some(&settings_layout), "Foreground Color");
    let fg_spinner = settings_ui.spinner(Some(&settings_layout));
    fg_spinner.set_list(&["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"]);

    settings_ui.default_label(Some(&settings_layout), "Cpu Speed");
    let cpu_box = settings_ui.edit_text(Some(&settings_layout), &cpu, true, true, false, "number");
    settings_ui.default_label(Some(&settings_layout), "Delay");
    let delay_box =
        settings_ui.edit_text(Some(&settings_layout), &delay, true, true, false, "number");
settings_ui.default_label(Some(&settings_layout), "Scale");
    let scale_box =
        settings_ui.edit_text(Some(&settings_layout), &scale, true, true, false, "number");
 
    let button_layout = settings_ui.linear_layout(Some(&layout), false);
    let save = settings_ui.button(Some(&button_layout), "Save");
    let close = settings_ui.button(Some(&button_layout), "Close");

    loop {
        let event = tgui.event();

        if event.ty == tgui::event::DESTROY
            && event.value["finishing"].to_string().trim().parse().unwrap()
        {
            break;
        }

        if event.ty == tgui::event::CLICK {
            if event.id == close.get_id() {
                settings_ui.activity.finish(&settings_ui.main);
            }
            if event.id == save.get_id() {
                cpu = cpu_box.get_text();
                delay = delay_box.get_text();
                scale = scale_box.get_text();
                config.with_section(Some("Hack")).set("cpu", &cpu);
                config.with_section(Some("Hack")).set("delay", &delay);
                config.with_section(Some("Screen")).set("scale", &scale);
                config.write_to_file("config/config.ini").unwrap();
            }
            if event.id == audio_button.get_id() {
                let audio = event.value["set"].to_string();
                config.with_section(Some("Audio")).set("enable", audio);
            }
        }

        if event.ty == tgui::event::ITEM_SELECTED {
            if event.id == bg_spinner.get_id() {
                let bg = event.value["selected"].to_string();
                let bg: Vec<u8> = bg
                    .as_bytes()
                    .iter()
                    .map(|&val| val)
                    .filter(|&val| val != b'\"')
                    .collect();
                let bg = String::from_utf8(bg).unwrap();
                config.with_section(Some("Theme")).set("bg", &bg);
            }

            if event.id == fg_spinner.get_id() {
                let fg = event.value["selected"].to_string();
                let fg: Vec<u8> = fg
                    .as_bytes()
                    .iter()
                    .map(|&val| val)
                    .filter(|&val| val != b'\"')
                    .collect();
                let fg = String::from_utf8(fg).unwrap();
                config.with_section(Some("Theme")).set("fg", &fg);
            }
        }
    }
}
