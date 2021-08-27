use super::super::CPU;
use super::super::MMU;

impl CPU {
    pub fn add_r(&mut self, opcode: u16) {
        let a = self.registers.a;
        let value = match opcode & 0xF {
            0x0 => self.registers.b,
            0x1 => self.registers.c,
            0x2 => self.registers.d,
            0x3 => self.registers.e,
            0x4 => self.registers.h,
            0x5 => self.registers.l,
            0x7 => self.registers.a,
            _ => panic!("you should not be here add r"),
        };

        self.registers.a = a.wrapping_add(value);

        self.registers.set_z_flag(self.registers.a == 0);
        self.registers.set_n_flag(false);
        self.registers.set_h_flag(((a & 0x0F) + (value & 0x0F)) > 0x0F);
        self.registers.set_c_flag(self.registers.a < a);

        self.clock += 4;
    }

    pub fn adc_r(&mut self, opcode: u16) {
        let a = self.registers.a;
        let mut value = match opcode & 0xF {
            0x8 => self.registers.b,
            0x9 => self.registers.c,
            0xA => self.registers.d,
            0xB => self.registers.e,
            0xC => self.registers.h,
            0xD => self.registers.l,
            0xF => self.registers.a,
            _ => panic!("you should not be here adc r"),
        };

        let c_flag = if self.registers.get_c_flag() { 1 } else { 0 };

        value = value.wrapping_add(c_flag);

        self.registers.a = a.wrapping_add(value);

        self.registers.set_z_flag(self.registers.a == 0);
        self.registers.set_n_flag(false);
        self.registers.set_h_flag(((a & 0x0F) + (value & 0x0F)) > 0x0F);
        self.registers.set_c_flag(self.registers.a < a);

        self.clock += 4;
    }

    pub fn add_rr(&mut self, opcode: u16, mmu: & MMU) {
        let a = self.registers.a;
        let value = match opcode {
            0x86 => mmu.read_byte(self.registers.get_hl()),
            0xC6 => { self.registers.pc += 1; mmu.read_byte(self.registers.pc - 1) },
            _ => panic!("You should not be here add rr"),
        };

        self.registers.a = a.wrapping_add(value);

        self.registers.set_z_flag(self.registers.a == 0);
        self.registers.set_n_flag(false);
        self.registers.set_h_flag(((a & 0x0F) + (value & 0x0F)) > 0x0F);
        self.registers.set_c_flag(self.registers.a < a);

        self.clock += 8;
    }

    pub fn add_hl_rr(&mut self, opcode: u16) {
        let hl = self.registers.get_hl();
        let value = match opcode {
            0x09 => self.registers.get_bc(),
            0x19 => self.registers.get_de(),
            0x29 => self.registers.get_hl(),
            0x39 => self.registers.sp,
            _ => panic!("You should not be here add hl rr"),
        };

        self.registers.set_hl(hl.wrapping_add(value));

        self.registers.set_n_flag(false);
        self.registers.set_h_flag((hl & 0x0FFF) + (value & 0x0FFF) > 0x0FFF);
        self.registers.set_c_flag(self.registers.get_hl() < hl);

        self.clock += 8;
    }

    pub fn adc_rr(&mut self, opcode: u16, mmu: & MMU) {
        let a = self.registers.a;
        let mut value = match opcode {
            0x8E => mmu.read_byte(self.registers.get_hl()),
            0xCE => { self.registers.pc += 1; mmu.read_byte(self.registers.pc - 1) },
            _ => panic!("You should not be here add rr"),
        };

        let c_flag = if self.registers.get_c_flag() { 1 } else { 0 };

        value = value.wrapping_add(c_flag);

        self.registers.a = a.wrapping_add(value);

        self.registers.set_z_flag(self.registers.a == 0);
        self.registers.set_n_flag(false);
        self.registers.set_h_flag(((a & 0x0F) + (value & 0x0F)) > 0x0F);
        self.registers.set_c_flag(self.registers.a < a);

        self.clock += 8;
    }

    pub fn add_sp_n(&mut self, mmu: & MMU) {
        let value = mmu.read_byte(self.registers.pc) as i8 as i16 as u16;
        self.registers.pc += 1;
        
        let sp = self.registers.sp;

        self.registers.sp = sp.wrapping_add(value);

        self.registers.set_z_flag(false);
        self.registers.set_n_flag(false);
        self.registers.set_h_flag((sp & 0x000F) + (value & 0x000F) > 0x000F);
        self.registers.set_c_flag((sp & 0x00FF) + (value & 0x00FF) > 0x00FF);

        self.clock += 16;
    }
}