use super::super::CPU;
use super::super::MMU;

impl CPU {
    pub fn ld_rr_nn(&mut self, opcode: u16, mmu: & MMU) {
        let valueue = mmu.read_word(self.registers.pc);
        self.registers.pc += 2;

        match opcode {
            0x01 => self.registers.set_bc(valueue),
            0x11 => self.registers.set_de(valueue),
            0x21 => self.registers.set_hl(valueue),
            0x31 => self.registers.sp = valueue,
            _ => panic!("You should not be here ld rr nn"),
        };
        
        self.clock += 12;
    }

    pub fn ld_rr_a(&mut self, opcode: u16, mmu: &mut MMU) {
        let addr = match opcode {
            0x02 => self.registers.get_bc(),
            0x12 => self.registers.get_de(),
            0x77 => self.registers.get_hl(),
            _ => panic!("You should not be here ld rr a"),
        };

        mmu.write_byte(addr, self.registers.a);
        self.clock += 8;
    }

    pub fn ld_r_n(&mut self, opcode: u16, mmu: & MMU) {
        let valueue = mmu.read_byte(self.registers.pc);
        self.registers.pc += 1;

        match opcode {
            0x06 => self.registers.b = valueue,
            0x0E => self.registers.c = valueue,
            0x16 => self.registers.d = valueue,
            0x1E => self.registers.e = valueue,
            0x26 => self.registers.h = valueue,
            0x2E => self.registers.l = valueue,
            0x3E => self.registers.a = valueue,
            _ => panic!("You should not be here ld r n"),
        }

        self.clock += 8
    }

    pub fn ld_a_rr(&mut self, opcode: u16, mmu: & MMU) {
        let addr = match opcode {
            0x0A => self.registers.get_bc(),
            0x1A => self.registers.get_de(),
            0x7E => self.registers.get_hl(),
            _ => panic!("You should not be here ld a rr"),
        };

        self.registers.a = mmu.read_byte(addr);
        self.clock += 8;
    }

    pub fn ld_hli_a(&mut self, mmu: &mut MMU) {
        let hl = self.registers.get_hl();
        mmu.write_byte(hl, self.registers.a);
        self.registers.set_hl(hl.wrapping_add(1));
        self.clock += 8;
    }

    pub fn ld_a_hli(&mut self, mmu: & MMU) {
        let hl = self.registers.get_hl();
        self.registers.a = mmu.read_byte(hl);
        self.registers.set_hl(hl.wrapping_add(1));
        self.clock += 8;
    }

    pub fn ld_hld_a(&mut self, mmu: &mut MMU) {
        let hl = self.registers.get_hl();
        mmu.write_byte(hl, self.registers.a);
        self.registers.set_hl(hl.wrapping_sub(1));
        self.clock += 8;
    }

    pub fn ld_a_hld(&mut self, mmu: & MMU) {
        let hl = self.registers.get_hl();
        self.registers.a = mmu.read_byte(hl);
        self.registers.set_hl(hl.wrapping_sub(1));
        self.clock += 8;
    }

    pub fn ld_hl_n(&mut self, mmu: &mut MMU) {
        mmu.write_byte(self.registers.get_hl(), mmu.read_byte(self.registers.pc));
        self.registers.pc += 1;
        self.clock += 12;
    }

    pub fn ld_r_r(&mut self, opcode: u16) {
        let value = match opcode & 0xF {
            0x0 | 0x8 => self.registers.b,
            0x1 | 0x9 => self.registers.c,
            0x2 | 0xA => self.registers.d,
            0x3 | 0xB => self.registers.e,
            0x4 | 0xC => self.registers.h,
            0x5 | 0xD => self.registers.l,
            0x7 | 0xF => self.registers.a,
            _ => panic!("you should not be here ld r r"),
        };

        match opcode {
            0x40..=0x47 => self.registers.b = value,
            0x48..=0x4F => self.registers.c = value,
            0x50..=0x57 => self.registers.d = value,
            0x58..=0x5F => self.registers.e = value,
            0x60..=0x67 => self.registers.h = value,
            0x68..=0x6F => self.registers.l = value,
            0x78..=0x7F => self.registers.a = value,
            _ => panic!("you should not be here ld r r"),
        }
        self.clock += 4;
    }

    pub fn ld_ion_a(&mut self, mmu: &mut MMU) {
        let value = mmu.read_byte(self.registers.pc) as u16;
        mmu.write_byte(0xFF00 + value, self.registers.a);
        self.registers.pc += 1;
        self.clock += 12;
    }

    pub fn ld_a_ion(&mut self, mmu: & MMU) {
        let value = mmu.read_byte(self.registers.pc) as u16;
        self.registers.a = mmu.read_byte(0xFF00 + value);
        self.registers.pc += 1;
        self.clock += 12;
    }

    pub fn ld_ioc_a(&mut self, mmu: &mut MMU) {
        let value = self.registers.c as u16;
        mmu.write_byte(0xFF00 + value, self.registers.a);
        self.clock += 8;
    }

    pub fn ld_a_ioc(&mut self, mmu: & MMU) {
        let value = self.registers.c as u16;
        self.registers.a = mmu.read_byte(0xFF00 + value);
        self.clock += 8;
    }

    pub fn ld_a_nn(&mut self, mmu: &mut MMU) {
        let addr = mmu.read_word(self.registers.pc);
        mmu.write_byte(addr, self.registers.a);
        self.registers.pc += 2;
        self.clock += 16;
    }
}