mod rom_only;
mod mbc1;

use rom_only::ROM_ONLY;
use mbc1::MBC1;

use std::path::PathBuf;
use std::fs;

pub trait MBC {
    // Constructor
    fn new(cartridge: Vec<u8>) -> Box<dyn MBC> where Self: Sized;

    // Method ROM ACCESS
    fn read_rom(&self, addr: u16) -> u8;
    fn write_rom(&mut self, addr: u16, value: u8);

    // Method RAM ACCESS
    fn read_ram(&self, addr: u16) -> u8;
    fn write_ram(&mut self, addr: u16, value: u8);

    // Default method definition
    fn print_cartridge_title(&self) {
        for i in 0x134..0x143 {
            print!("{}", self.read_rom(i) as char);
        }
        println!();
    }
}

impl Default for Box<dyn MBC> {
    fn default() -> Box<dyn MBC> {
        return ROM_ONLY::new(vec!());
    }
}

pub fn get_mbc(cartridge_path: PathBuf) -> Box<dyn MBC> {
    let rom = fs::read(cartridge_path).expect("Could not read file");

    match rom[0x147] {
        0x00 => { println!("ROM ONLY"); ROM_ONLY::new(rom) },
        0x01 => { println!("MBC1"); MBC1::new(rom) },
        _ => panic!("MBC required for this cartridge is not implemented!"),
    }
}

pub fn nb_ram_bank(value: u8) -> usize {
    return match value {
        2 => 1,
        3 => 4,
        4 => 16,
        5 => 8,
        _ => 0,
    };
}