use super::super::CPU;
use super::super::MMU;

impl CPU {
    pub fn bit_b_r(&mut self, opcode: u16) {
        let value = match opcode & 0xF {
            0x0 | 0x8 => self.registers.b,
            0x1 | 0x9 => self.registers.c,
            0x2 | 0xA => self.registers.d,
            0x3 | 0xB => self.registers.e,
            0x4 | 0xC => self.registers.h,
            0x5 | 0xD => self.registers.l,
            0x7 | 0xF => self.registers.a,
            _ => panic!("you should not be here bit b r"),
        };

        let bit = match opcode & 0xFF {
            0x40..=0x47 => 0x01,
            0x48..=0x4F => 0x02,
            0x50..=0x57 => 0x04,
            0x58..=0x5F => 0x08,
            0x60..=0x67 => 0x10,
            0x68..=0x6F => 0x20,
            0x70..=0x77 => 0x40,
            0x78..=0x7F => 0x80,
            _ => panic!("you should not be here bit b r"),
        };

        self.registers.set_z_flag((value & bit) == 0);
        self.registers.set_n_flag(false);
        self.registers.set_h_flag(true);

        self.clock += 8;
    }

    pub fn bit_b_hl(&mut self, opcode: u16, mmu: &mut MMU) {
        let hl = self.registers.get_hl();
        let value = mmu.read_byte(hl);

        let bit = match opcode & 0xFF {
            0x40..=0x47 => 0x01,
            0x48..=0x4F => 0x02,
            0x50..=0x57 => 0x04,
            0x58..=0x5F => 0x08,
            0x60..=0x67 => 0x10,
            0x68..=0x6F => 0x20,
            0x70..=0x77 => 0x40,
            0x78..=0x7F => 0x80,
            _ => panic!("you should not be here bit b hl"),
        };

        self.registers.set_z_flag((value & bit) == 0);
        self.registers.set_n_flag(false);
        self.registers.set_h_flag(true);

        self.clock += 16;
    }
}