use super::MBC;

#[allow(non_camel_case_types)]
pub struct ROM_ONLY {
    rom: Vec<u8>,
}

impl MBC for ROM_ONLY {
    fn new(rom: Vec<u8>) -> Box<dyn MBC> {
        return Box::new(ROM_ONLY { rom: rom, } );
    }

    fn read_rom(&self, addr: u16) -> u8 { return self.rom[addr as usize]; }
    fn write_rom(&mut self, _addr: u16, _value: u8) { return ; }

    fn read_ram(&self, addr: u16) -> u8 { return 0; }
    fn write_ram(&mut self, _addr: u16, _value: u8) { return ; }
}