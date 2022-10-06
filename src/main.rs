mod core;
mod frontend;

use frontend::minifb::emulator::Emulator;

fn main() {
    let mut app = Emulator::new();
    app.run();
}
