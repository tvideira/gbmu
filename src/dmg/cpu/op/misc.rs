use super::super::CPU;
use super::super::MMU;

impl CPU {
    pub fn nop(&mut self) {
        self.clock += 4
    }

    pub fn stop(&mut self) {
        self.is_halted_until_button_pressed = true;
        self.registers.pc += 1;
        self.clock += 4;
    }

    pub fn daa(&mut self) {
        let n_flag = self.registers.get_n_flag();
        let h_flag = self.registers.get_h_flag();
        let c_flag = self.registers.get_c_flag();

        if !n_flag {
            if c_flag || self.registers.a > 0x99 {
                self.registers.a = self.registers.a.wrapping_add(0x60);
                self.registers.set_c_flag(true);
            }
            if h_flag || (self.registers.a & 0x0f) > 0x09 {
                self.registers.a = self.registers.a.wrapping_add(0x06);
            }
        } else {
            if c_flag {
                self.registers.a = self.registers.a.wrapping_sub(0x60);
            }
            if h_flag {
                self.registers.a = self.registers.a.wrapping_sub(0x06);
            }
        }
        self.registers.set_z_flag(self.registers.a == 0);
        self.registers.set_h_flag(false);
        self.registers.pc += 1;
        self.clock += 4;
    }

    pub fn cpl(&mut self) {
        self.registers.a ^= 0xFF;
        self.registers.set_n_flag(true);
        self.registers.set_h_flag(true);
        self.clock += 4;
    }

    pub fn halt(&mut self) {
        self.is_halted = true;
        self.clock += 4;
    }

    pub fn rst(&mut self, opcode: u16, mmu: &mut MMU) {
        self.registers.sp -= 2;
        mmu.write_word(self.registers.sp, self.registers.pc);
        self.registers.pc = match opcode {
            0xC7 => 0x00,
            0xCF => 0x08,
            0xD7 => 0x10,
            0xDF => 0x18,
            0xE7 => 0x20,
            0xEF => 0x28,
            0xF7 => 0x30,
            0xFF => 0x38,
            _ => panic!("You should not be here rst"),
        };

        self.clock += 16;
    }
}