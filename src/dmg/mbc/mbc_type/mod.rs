mod rom_only;

use super::MBC;
use super::MMU;

use rom_only::update_banks_rom_only;

use std::fmt;

pub enum MBC_TYPE {
    ROM_ONLY,
    MBC1,
    MBC2,
    MBC3,
    MBC5,
}

impl Default for MBC_TYPE {
    fn default() -> Self {
        Self::ROM_ONLY
    }
}

impl fmt::Display for MBC_TYPE {
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

impl MBC for MBC_TYPE {
    fn update_banks(&self, mmu: &mut MMU, cartridge: &Vec<u8>) {
        match self {
            Self::ROM_ONLY => update_banks_rom_only(mmu, cartridge),
            _ => panic!("CARTRIDGE MBC NOT IMPLEMENTED YET"),
        }
        
    }
}

pub fn get_mbc(cartridge: &Vec<u8>) -> MBC_TYPE {
    match cartridge[0x147] {
        0x00 => MBC_TYPE::ROM_ONLY,
        0x01 => MBC_TYPE::MBC1,
        0x05 => MBC_TYPE::MBC2,
        0x11 => MBC_TYPE::MBC3,
        0x19 => MBC_TYPE::MBC5,
        _ => panic!("CARTRIDGE MBC NOT IMPLEMENTED YET"),
    }
} 