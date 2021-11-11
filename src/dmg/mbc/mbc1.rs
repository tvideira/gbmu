use super::{MBC, nb_ram_bank};

#[allow(non_camel_case_types)]
pub struct MBC1 {
    rom: Vec<u8>,
    ram: Vec<u8>,
    ram_enabled: bool,
    ram_bank_mode: bool,
    rom_bank: usize,
    ram_bank: usize,
}

impl MBC for MBC1 {
    fn new(rom: Vec<u8>) -> Box<dyn MBC> {
        let mut mbc = Box::new(MBC1 { 
                rom: rom, ram: vec![],
                ram_enabled: false,
                ram_bank_mode: false,
                rom_bank: 1,
                ram_bank: 0,
            }
        );
        if mbc.rom.len() > 0x149 {
            mbc.ram = vec![0; nb_ram_bank(mbc.rom[0x149]) * 8192];
        }
        return mbc;
    }

    fn read_rom(&self, addr: u16) -> u8 {
        if addr < 0x4000 {
            return self.rom[addr as usize];
        } else {
            return self.rom[((0x4000 * self.rom_bank) + (addr & 0x3FFF) as usize)];
        }
    }

    // TODO
    fn write_rom(&mut self, _addr: u16, _value: u8) {
        return ;
    }

    fn read_ram(&self, addr: u16) -> u8 { 
        if !self.ram_enabled {
            return 0;
        } 
        let ram_bank = if self.ram_bank_mode { self.ram_bank } else { 0 };
        return self.ram[((0x2000 * ram_bank) + (addr & 0x1FFF) as usize)];
    }

    fn write_ram(&mut self, addr: u16, value: u8) {
        if !self.ram_enabled {
            return ;
        }
        let ram_bank = if self.ram_bank_mode { self.ram_bank } else { 0 };
        self.ram[((0x2000 * ram_bank) + (addr & 0x1FFF) as usize)] = value;
    }
}