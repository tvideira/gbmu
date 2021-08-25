mod cpu;
mod mmu;
mod ppu;
mod mbc;

use cpu::CPU;
use mmu::MMU;
use ppu::PPU;
use mbc::MBC;
use mbc::mbc_type::MBC_TYPE;
use mbc::mbc_type::get_mbc;

use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

// GAME BOY STRUCT
#[derive(Default)]
pub struct DMG {
    cpu: CPU,
    mmu: MMU,
    ppu: PPU,
    mbc: MBC_TYPE,
    cartridge: Vec<u8>,
}

impl DMG {
    pub fn load_cartridge(&mut self, cartridge_path: PathBuf) {
        let mut file = match File::open(cartridge_path) {
            Ok(file) => file,
            Err(why) => panic!("couldn't open file because: {}", why),
        };
    
        let _size = match file.read_to_end(&mut self.cartridge) {
            Ok(_size) => _size,
            Err(why) => panic!("couldn't read file because: {}", why),
        };

        self.mbc = get_mbc(&self.cartridge);
        println!("{}", self.mbc);

        self.mmu.disable_bios();
        self.mbc.update_banks(&mut self.mmu, &self.cartridge);
        self.mmu.enable_bios();
    }

    pub fn run(&mut self) {
        self.mbc.print_cartridge_title(&self.cartridge);
    }
}