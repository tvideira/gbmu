use super::super::CPU;
use super::super::MMU;

impl CPU {
    pub fn call(&mut self, opcode: u16, mmu: &mut MMU) {
        let case = match opcode {
            0xCD => true,
            0xC4 => !self.registers.get_z_flag(),
            0xCC => self.registers.get_z_flag(),
            0xD4 => !self.registers.get_c_flag(),
            0xDC => self.registers.get_c_flag(),
            _ => panic!("You should not be here call"),
        };

        if case {
            self.registers.sp -= 2;
            mmu.write_word(self.registers.sp, self.registers.pc + 2);
            self.registers.pc = mmu.read_word(self.registers.pc);
            self.clock += 24;
        } else {
            self.registers.pc += 2;
            self.clock += 12;
        }

    }
}