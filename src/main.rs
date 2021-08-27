mod dmg;

use dmg::DMG;

use native_dialog::FileDialog;

fn main() {
    // Opening explorer to choose a rom to load
    let cartridge_path = match FileDialog::new().show_open_single_file().unwrap() {
        Some(path) => path,
        None => return,
    };

    // Creating a emulator instance
    let mut dmg: DMG = Default::default();

    dmg.load_cartridge(cartridge_path);
    dmg.start();
}
