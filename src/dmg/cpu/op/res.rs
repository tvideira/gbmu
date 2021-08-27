use super::super::CPU;
use super::super::MMU;

impl CPU {
    pub fn res_b_r(&mut self, opcode: u16) {
        let bit = match opcode & 0xFF {
            0x80..=0x87 => 0xFE,
            0x88..=0x8F => 0xFD,
            0x90..=0x97 => 0xFB,
            0x98..=0x9F => 0xF7,
            0xA0..=0xA7 => 0xEF,
            0xA8..=0xAF => 0xDF,
            0xB0..=0xB7 => 0xBF,
            0xB8..=0xBF => 0x7F,
            _ => panic!("you should not be here res b r"),
        };

        match opcode & 0xF {
            0x0 | 0x8 => self.registers.b &= bit,
            0x1 | 0x9 => self.registers.c &= bit,
            0x2 | 0xA => self.registers.d &= bit,
            0x3 | 0xB => self.registers.e &= bit,
            0x4 | 0xC => self.registers.h &= bit,
            0x5 | 0xD => self.registers.l &= bit,
            0x7 | 0xF => self.registers.a &= bit,
            _ => panic!("you should not be here res b r"),
        };

        self.clock += 8;
    }

    pub fn res_b_hl(&mut self, opcode: u16, mmu: &mut MMU) {
        let hl = self.registers.get_hl();
        let value = mmu.read_byte(hl);

        let bit = match opcode & 0xFF {
            0x80..=0x87 => 0xFE,
            0x88..=0x8F => 0xFD,
            0x90..=0x97 => 0xFB,
            0x98..=0x9F => 0xF7,
            0xA0..=0xA7 => 0xEF,
            0xA8..=0xAF => 0xDF,
            0xB0..=0xB7 => 0xBF,
            0xB8..=0xBF => 0x7F,
            _ => panic!("you should not be here res b r"),
        };

        mmu.write_byte(hl, value & bit);

        self.clock += 16;
    }
}