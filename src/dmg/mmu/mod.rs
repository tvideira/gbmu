// MEMORY MANAGMEMENT UNIT STRUCT

pub struct MMU {
    is_bios_enabled:    bool,
    bios:               Vec<u8>,
    rom_bank_0:         Vec<u8>,
    rom_bank_1:         Vec<u8>,
    tile_data:          Vec<Vec<u8>>, // video ram 0x8000 ~ 0x97FF
    tile_maps:          Vec<Vec<u8>>, // video ram 0x9800 ~ 0x9FFF
    external_ram:       Vec<u8>,
    working_ram:        Vec<u8>,
    oam:                Vec<u8>,
    io_registers:       Vec<u8>,
    high_ram:           Vec<u8>,
    ie_register:        u8,
}

impl Default for MMU {
    fn default() -> Self {
        Self {
            is_bios_enabled: false,
            bios: vec![
                0x31, 0xFE, 0xFF, 0xAF, 0x21, 0xFF, 0x9F, 0x32, 0xCB, 0x7C, 0x20, 0xFB, 0x21, 0x26, 0xFF, 0x0E,
                0x11, 0x3E, 0x80, 0x32, 0xE2, 0x0C, 0x3E, 0xF3, 0xE2, 0x32, 0x3E, 0x77, 0x77, 0x3E, 0xFC, 0xE0,
                0x47, 0x11, 0x04, 0x01, 0x21, 0x10, 0x80, 0x1A, 0xCD, 0x95, 0x00, 0xCD, 0x96, 0x00, 0x13, 0x7B,
                0xFE, 0x34, 0x20, 0xF3, 0x11, 0xD8, 0x00, 0x06, 0x08, 0x1A, 0x13, 0x22, 0x23, 0x05, 0x20, 0xF9,
                0x3E, 0x19, 0xEA, 0x10, 0x99, 0x21, 0x2F, 0x99, 0x0E, 0x0C, 0x3D, 0x28, 0x08, 0x32, 0x0D, 0x20,
                0xF9, 0x2E, 0x0F, 0x18, 0xF3, 0x67, 0x3E, 0x64, 0x57, 0xE0, 0x42, 0x3E, 0x91, 0xE0, 0x40, 0x04,
                0x1E, 0x02, 0x0E, 0x0C, 0xF0, 0x44, 0xFE, 0x90, 0x20, 0xFA, 0x0D, 0x20, 0xF7, 0x1D, 0x20, 0xF2,
                0x0E, 0x13, 0x24, 0x7C, 0x1E, 0x83, 0xFE, 0x62, 0x28, 0x06, 0x1E, 0xC1, 0xFE, 0x64, 0x20, 0x06,
                0x7B, 0xE2, 0x0C, 0x3E, 0x87, 0xF2, 0xF0, 0x42, 0x90, 0xE0, 0x42, 0x15, 0x20, 0xD2, 0x05, 0x20,
                0x4F, 0x16, 0x20, 0x18, 0xCB, 0x4F, 0x06, 0x04, 0xC5, 0xCB, 0x11, 0x17, 0xC1, 0xCB, 0x11, 0x17,
                0x05, 0x20, 0xF5, 0x22, 0x23, 0x22, 0x23, 0xC9, 0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B,
                0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E,
                0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC,
                0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E, 0x3c, 0x42, 0xB9, 0xA5, 0xB9, 0xA5, 0x42, 0x3C,
                0x21, 0x04, 0x01, 0x11, 0xA8, 0x00, 0x1A, 0x13, 0xBE, 0x20, 0xFE, 0x23, 0x7D, 0xFE, 0x34, 0x20,
                0xF5, 0x06, 0x19, 0x78, 0x86, 0x23, 0x05, 0x20, 0xFB, 0x86, 0x20, 0xFE, 0x3E, 0x01, 0xE0, 0x50
            ],
            rom_bank_0:     vec!(0; 0x4000),
            rom_bank_1:     vec!(0; 0x4000),
            tile_data:      vec!(vec!(0; 16); 384),
            tile_maps:      vec!(vec!(0; 32); 64),
            external_ram:   vec!(0; 0x2000),
            working_ram:    vec!(0; 0x2000),
            oam:            vec!(0; 0x00A0),
            io_registers:   vec!(0; 0x0080),
            high_ram:       vec!(0; 0x7F),
            ie_register:    0,
        }
    }
}

impl MMU {
    pub fn read_byte(&self, addr: u16) -> u8 {
        return match addr {
            // BIOS if still loaded, otherwise ROM BANK 0
            0x0000 ..= 0x00FF => {
                if self.is_bios_enabled {
                    self.bios[addr as usize]
                } else {
                    self.rom_bank_0[addr as usize]
                }
            },
            
            // ROM BANK 0
            0x0100 ..= 0x3FFF => self.rom_bank_0[addr as usize],
            
            // ROM BANK 01 ~ NN
            0x4000 ..= 0x7FFF => self.rom_bank_1[(addr - 0x4000) as usize],
            
            // VIDEO RAM
            0x8000 ..= 0x97FF => self.tile_data[((addr - 0x8000) / 16) as usize][((addr - 0x8000) % 16) as usize],
            0x9800 ..= 0x9FFF => self.tile_maps[((addr - 0x9800) / 32) as usize][((addr - 0x9800) % 32) as usize],
            
            // EXTERNAL RAM
            0xA000 ..= 0xBFFF => self.external_ram[(addr & 0x1FFF) as usize],
            
            // WORKING RAM
            0xC000 ..= 0xDFFF => self.working_ram[(addr & 0x1FFF) as usize],
            
            // ECHO RAM, NINTENDO SAYS THIS AREA IS PROHIBITED
            0xE000 ..= 0xFDFF => self.working_ram[(addr & 0x1FFF) as usize],
            
            // OAM
            0xFE00 ..= 0xFE9F => self.oam[(addr & 0x00FF) as usize],
            
            // NINTENDO SAYS THIS AREA IS PROHIBITED
            0xFEA0 ..= 0xFEFF => 0x00,
            
            // I/O REGISTERS
            0xFF00 ..= 0xFF7F => self.io_registers[(addr & 0x00FF) as usize],
            
            // HIGH RAM
            0xFF80 ..= 0xFFFE => self.high_ram[(addr & 0x007F) as usize],
            
            // INTERRUPT ENABLE REGISTER
            0xFFFF => return self.ie_register,
        };
    }

    pub fn read_word(&self, addr: u16) -> u16 {
        return ((self.read_byte(addr + 1) as u16) << 8) | (self.read_byte(addr) as u16);
    }

    pub fn write_byte(&mut self, addr: u16, val: u8) {
        match addr {
            // BIOS if still loaded, otherwise ROM BANK 0
            0x0000 ..= 0x00FF => {
                if self.is_bios_enabled {
                    self.bios[addr as usize] = val;
                } else {
                    self.rom_bank_0[addr as usize] = val;
                }
            },
            
            // ROM BANK 0
            0x0100 ..= 0x3FFF => self.rom_bank_0[addr as usize] = val,
            
            // ROM BANK 01 ~ NN
            0x4000 ..= 0x7FFF => self.rom_bank_1[(addr - 0x4000) as usize] = val,
            
            // VIDEO RAM
            0x8000 ..= 0x97FF => self.tile_data[((addr - 0x8000) / 16) as usize][((addr - 0x8000) % 16) as usize] = val,
            0x9800 ..= 0x9FFF => self.tile_maps[((addr - 0x9800) / 32) as usize][((addr - 0x9800) % 32) as usize] = val,
            
            // EXTERNAL RAM
            0xA000 ..= 0xBFFF => self.external_ram[(addr & 0x1FFF) as usize] = val,
            
            // WORKING RAM
            0xC000 ..= 0xDFFF => self.working_ram[(addr & 0x1FFF) as usize] = val,
            
            // ECHO RAM, NINTENDO SAYS THIS AREA IS PROHIBITED
            0xE000 ..= 0xFDFF => self.working_ram[(addr & 0x1FFF) as usize] = val,
            
            // OAM
            0xFE00 ..= 0xFE9F => self.oam[(addr & 0x00FF) as usize] = val,
            
            // NINTENDO SAYS THIS AREA IS PROHIBITED
            0xFEA0 ..= 0xFEFF => return,
            
            // I/O REGISTERS
            0xFF00 ..= 0xFF7F => self.io_registers[(addr & 0x00FF) as usize] = val,
            
            // HIGH RAM
            0xFF80 ..= 0xFFFE => self.high_ram[(addr & 0x007F) as usize] = val,
            
            // INTERRUPT ENABLE REGISTER
            0xFFFF => self.ie_register = val,
        }
    }

    pub fn get_tile_data(&self, index: usize) -> & Vec<u8> {
        return &self.tile_data[index];
    }

    pub fn write_word(&mut self, addr: u16, val: u16) {
        self.write_byte(addr, (val & 0x00FF) as u8);
        self.write_byte(addr + 1, ((val & 0xFF00) >> 8) as u8);
    }

    pub fn enable_bios(&mut self) {
        self.is_bios_enabled = true;
    }

    pub fn disable_bios(&mut self) {
        self.is_bios_enabled = false;
    }
}
