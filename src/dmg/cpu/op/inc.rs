use super::super::CPU;
use super::super::MMU;

impl CPU {
    pub fn inc_r(&mut self, opcode: u16) {
        let value: u8;
        match opcode {
            0x04 => { value = self.registers.b; self.registers.b = self.registers.b.wrapping_add(1); },
            0x0C => { value = self.registers.c; self.registers.c = self.registers.c.wrapping_add(1); },
            0x14 => { value = self.registers.d; self.registers.d = self.registers.d.wrapping_add(1); },
            0x1C => { value = self.registers.e; self.registers.e = self.registers.e.wrapping_add(1); },
            0x24 => { value = self.registers.h; self.registers.h = self.registers.h.wrapping_add(1); },
            0x2C => { value = self.registers.l; self.registers.l = self.registers.l.wrapping_add(1); },
            0x3C => { value = self.registers.a; self.registers.a = self.registers.a.wrapping_add(1); },
            _ => panic!("You should not be here inc r"),
        };
        
        self.registers.set_n_flag(false);
        self.registers.set_h_flag((value & 0x0F) == 0x0F);
        self.registers.set_z_flag(value.wrapping_add(1) == 0);
        self.clock += 4;
    }

    pub fn inc_rr(&mut self, opcode: u16) {
        match opcode {
            0x03 => self.registers.set_bc(self.registers.get_bc().wrapping_add(1)),
            0x13 => self.registers.set_de(self.registers.get_de().wrapping_add(1)),
            0x23 => self.registers.set_hl(self.registers.get_hl().wrapping_add(1)),
            0x33 => self.registers.sp = self.registers.sp.wrapping_add(1),
            _ => panic!("You should not be here inc rr"),
        };
        self.clock += 8;
    }
}