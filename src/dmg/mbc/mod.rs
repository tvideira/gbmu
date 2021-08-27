mod rom_only;

use super::MMU;

use rom_only::update_banks_rom_only;

use std::fmt;

#[allow(non_camel_case_types)]
pub enum MBC {
    ROM_ONLY,
    MBC1,
    MBC2,
    MBC3,
    MBC5,
}

impl Default for MBC {
    fn default() -> Self {
        Self::ROM_ONLY
    }
}

impl fmt::Display for MBC {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ROM_ONLY => { write!(f, "ROM_ONLY") },
            Self::MBC1 => { write!(f, "MBC1") },
            Self::MBC2 => { write!(f, "MBC2") },
            Self::MBC3 => { write!(f, "MBC3") },
            Self::MBC5 => { write!(f, "MBC5") },
        }
    }
}

impl MBC {
    pub fn update_banks(&self, mmu: &mut MMU, cartridge: &Vec<u8>) {
        match self {
            Self::ROM_ONLY => update_banks_rom_only(mmu, cartridge),
            _ => panic!("CARTRIDGE MBC NOT IMPLEMENTED YET"),
        }        
    }
}

pub fn get_mbc(cartridge: &Vec<u8>) -> MBC {
    match cartridge[0x147] {
        0x00 => MBC::ROM_ONLY,
        _ => MBC::ROM_ONLY,
    }
} 