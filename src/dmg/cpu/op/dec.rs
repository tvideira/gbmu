use super::super::CPU;
use super::super::MMU;

impl CPU {
    pub fn dec_r(&mut self, opcode: u16) {
        let value: u8;
        match opcode {
            0x05 => { value = self.registers.b; self.registers.b = self.registers.b.wrapping_sub(1); },
            0x0D => { value = self.registers.c; self.registers.c = self.registers.c.wrapping_sub(1); },
            0x15 => { value = self.registers.d; self.registers.d = self.registers.d.wrapping_sub(1); },
            0x1D => { value = self.registers.e; self.registers.e = self.registers.e.wrapping_sub(1); },
            0x25 => { value = self.registers.h; self.registers.h = self.registers.h.wrapping_sub(1); },
            0x2D => { value = self.registers.l; self.registers.l = self.registers.l.wrapping_sub(1); },
            0x3D => { value = self.registers.a; self.registers.a = self.registers.a.wrapping_sub(1); },
            _ => panic!("You should not be here dec r"),
        };
        
        self.registers.set_n_flag(true);
        self.registers.set_h_flag((value & 0x0F) == 0x00);
        self.registers.set_z_flag(value.wrapping_sub(1) == 0);
        self.clock += 4;
    }

    pub fn dec_rr(&mut self, opcode: u16) {
        match opcode {
            0x0B => self.registers.set_bc(self.registers.get_bc().wrapping_sub(1)),
            0x1B => self.registers.set_de(self.registers.get_de().wrapping_sub(1)),
            0x2B => self.registers.set_hl(self.registers.get_hl().wrapping_sub(1)),
            0x3B => self.registers.sp = self.registers.sp.wrapping_sub(1),
            _ => panic!("You should not be here dec rr"),
        };
        self.clock += 8;
    }

    pub fn dec_hl(&mut self, mmu: &mut MMU) {
        let hl = self.registers.get_hl();
        let value = mmu.read_byte(hl);
        mmu.write_byte(hl, value.wrapping_sub(1));
        
        self.registers.set_n_flag(true);
        self.registers.set_h_flag((value & 0x0F) == 0x00);
        self.registers.set_z_flag(value.wrapping_sub(1) == 0);
        self.clock += 12;
    }
}