mod cpu;
mod mmu;
mod ppu;
mod mbc;

use cpu::CPU;
use mmu::MMU;
use ppu::PPU;
use mbc::MBC;

use std::path::PathBuf;

// GAME BOY STRUCT
#[derive(Default)]
pub struct DMG {
    cpu: CPU,
    mmu: MMU,
    ppu: PPU,
    mbc: Box<dyn MBC>,
}

impl DMG {
    pub fn load_cartridge(&mut self, cartridge_path: PathBuf) {
        self.mbc.load_cartridge(cartridge_path);
    }

    pub fn run(&mut self) {
        self.mbc.print_cartridge_title();
    }
}