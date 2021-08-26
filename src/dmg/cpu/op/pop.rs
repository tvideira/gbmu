use super::super::CPU;
use super::super::MMU;

impl CPU {
    pub fn pop(&mut self, opcode: u16, mmu: & MMU) {
        let value = mmu.read_word(self.registers.sp);
        
        match opcode {
            0xC1 => self.registers.set_bc(value),
            0xD1 => self.registers.set_de(value),
            0xE1 => self.registers.set_hl(value),
            0xF1 => self.registers.set_af(value),
            _ => panic!("You should not be here pop"),
        };
        
        self.registers.sp += 2;
        self.clock += 12;
    }
}