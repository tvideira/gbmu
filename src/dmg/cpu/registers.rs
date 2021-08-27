// REGISTERS FOR CPU
#[derive(Default)]
pub struct REGISTERS {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16,
}

impl REGISTERS {
    pub fn get_af(&self) -> u16 {
        return ((self.a as u16) << 8) | (self.f as u16);
    }

    pub fn get_bc(&self) -> u16 {
        return ((self.b as u16) << 8) | (self.c as u16);
    }

    pub fn get_de(&self) -> u16 {
        return ((self.d as u16) << 8) | (self.e as u16);
    }

    pub fn get_hl(&self) -> u16 {
        return ((self.h as u16) << 8) | (self.l as u16);
    }

    pub fn set_af(&mut self, val: u16) {
        self.a = (val >> 8) as u8;
        self.f = (val & 0xF0) as u8;
    }

    pub fn set_bc(&mut self, val: u16) {
        self.b = (val >> 8) as u8;
        self.c = (val & 0xFF) as u8;
    }

    pub fn set_de(&mut self, val: u16) {
        self.d = (val >> 8) as u8;
        self.e = (val & 0xFF) as u8;
    }

    pub fn set_hl(&mut self, val: u16) {
        self.h = (val >> 8) as u8;
        self.l = (val & 0xFF) as u8;
    }

    pub fn set_z_flag(&mut self, val: bool) {
        if val {
            self.f |= 0x80;
        } else {
            self.f &= 0x7F;  
        }
    }

    pub fn set_n_flag(&mut self, val: bool) {
        if val {
            self.f |= 0x40;
        } else {
            self.f &= 0xBF;  
        }
    }

    pub fn set_h_flag(&mut self, val: bool) {
        if val {
            self.f |= 0x20;
        } else {
            self.f &= 0xDF;  
        }
    }

    pub fn set_c_flag(&mut self, val: bool) {
        if val {
            self.f |= 0x10;
        } else {
            self.f &= 0xEF;  
        }
    }

    pub fn get_z_flag(&self) -> bool {
        return (self.f & 0x80) == 0x80;
    }

    pub fn get_n_flag(&self) -> bool {
        return (self.f & 0x40) == 0x40;
    }

    pub fn get_h_flag(&self) -> bool {
        return (self.f & 0x20) == 0x20;
    }

    pub fn get_c_flag(&self) -> bool {
        return (self.f & 0x10) == 0x10;
    }
}