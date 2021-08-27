use super::super::CPU;
use super::super::MMU;

impl CPU {
    pub fn jr_n(&mut self, opcode: u16, mmu: & MMU) {
        let value = mmu.read_byte(self.registers.pc);
        self.registers.pc += 1;

        let case = match opcode {
            0x18 => true,
            0x20 => !self.registers.get_z_flag(),
            0x28 => self.registers.get_z_flag(),
            0x30 => !self.registers.get_c_flag(),
            0x38 => self.registers.get_c_flag(),
            _ => panic!("You should not be here jr n"),
        };

        if case {
            if value > 127 {
                self.registers.pc -= (255 - value + 1) as u16;
            } else {
                self.registers.pc += value as u16;
            }
            self.clock += 12
        } else {
            self.clock += 8;
        }
    }

    pub fn jp_nn(&mut self, opcode: u16, mmu: & MMU) {
        
        let case = match opcode {
            0xC3 => true,
            0xC2 => !self.registers.get_z_flag(),
            0xCA => self.registers.get_z_flag(),
            0xD2 => !self.registers.get_c_flag(),
            0xDA => self.registers.get_c_flag(),
            _ => panic!("You should not be here jp nn"),
        };

        if case {
            self.registers.pc = mmu.read_word(self.registers.pc);
            self.clock += 16;
        } else {
            self.registers.pc += 2;
            self.clock += 12
        }
    }

    pub fn jp_hl(&mut self) {
        self.registers.pc = self.registers.get_hl();
        self.clock += 4;
    }
}