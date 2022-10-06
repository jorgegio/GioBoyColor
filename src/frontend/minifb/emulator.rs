use std::ffi::OsStr;
use std::path::PathBuf;

use minifb::MENU_KEY_CTRL;
use minifb::{InputCallback, Key, Menu, Window, WindowOptions};

use rfd::FileDialog;

use crate::core::gbc::GioBoyColor;

const WIDTH: usize = 160;
const HEIGHT: usize = 144;

const FILE_OPEN_MENU_ID: usize = 1;
const FILE_CLOSE_MENU_ID: usize = 2;

struct KeyCharCallback;

impl InputCallback for KeyCharCallback {
    fn add_char(&mut self, c: u32) {
        println!("add_char {}", c);
    }
}

pub struct Emulator {
    window: Window,
    gbc: GioBoyColor,
}

impl Emulator {
    pub fn new() -> Emulator {
        let mut window = Window::new(
            "GioBoyColor",
            WIDTH,
            HEIGHT,
            WindowOptions {
                resize: false,
                scale: minifb::Scale::X2,
                ..WindowOptions::default()
            },
        )
        .expect("Unable to Open Window");

        window.set_input_callback(Box::new(KeyCharCallback {}));

        let mut file_menu: Menu = Menu::new("File").unwrap();

        file_menu
            .add_item("Open GBC...", FILE_OPEN_MENU_ID)
            .shortcut(Key::O, MENU_KEY_CTRL)
            .build();

        file_menu.add_item("Close ROM", FILE_CLOSE_MENU_ID).build();

        window.add_menu(&file_menu);

        let gbc = GioBoyColor::new();

        return Emulator { window, gbc };
    }
    pub fn run(&mut self) {
        // TODO: Get buffer from GBC Core
        let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

        // Limit to max ~60 fps update rate
        self.window
            .limit_update_rate(Some(std::time::Duration::from_micros(16600)));

        while self.window.is_open() {
            self.handle_menus();

            // TODO: Update buffer from GBC Core
            for i in buffer.iter_mut() {
                *i = 0; // write something more funny here!
            }

            // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
            self.window
                .update_with_buffer(&buffer, WIDTH, HEIGHT)
                .unwrap();
        }
    }
    fn handle_menus(&mut self) {
        if let Some(menu_id) = self.window.is_menu_pressed() {
            match menu_id {
                FILE_OPEN_MENU_ID => {
                    let file = FileDialog::new()
                        .add_filter("Game Boy Color Rom", &["gbc", "rs"])
                        .set_directory("/")
                        .pick_file();
                    match file {
                        Some(filepath) => {
                            self.load_rom(&filepath);
                        }
                        None => println!("Didn't pick a file"),
                    }
                }
                FILE_CLOSE_MENU_ID => self.unload_rom(),
                _ => (),
            }
        }
    }
    fn load_rom(&mut self, rom_path: &PathBuf) {
        // Update window title
        let filename = rom_path.file_name().and_then(OsStr::to_str);

        match filename {
            Some(filename_str) => {
                self.gbc.load_rom(rom_path);
                let window_title = format!("GioBoyColor - {}", &filename_str);
                self.window.set_title(&window_title);
            }
            None => self.window.set_title("GioBoyColor"),
        }
    }
    fn unload_rom(&mut self) {
        self.window.set_title("GioBoyColor");
    }
}
