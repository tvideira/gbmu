use super::super::CPU;
use super::super::MMU;

impl CPU {
    pub fn ret(&mut self, opcode: u16, mmu: &mut MMU) {
        let case = match opcode {
            0xC9 => true,
            0xC0 => !self.registers.get_z_flag(),
            0xC8 => self.registers.get_z_flag(),
            0xD0 => !self.registers.get_c_flag(),
            0xD8 => self.registers.get_c_flag(),
            _ => panic!("You should not be here call"),
        };

        if case {
            self.registers.pc = mmu.read_word(self.registers.sp);
            self.registers.sp += 2;
            self.clock += if opcode == 0xC9 { 16 } else { 20 } ;
        } else {
            self.clock += 8;
        }

    }
}