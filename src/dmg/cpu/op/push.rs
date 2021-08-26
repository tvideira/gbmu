use super::super::CPU;
use super::super::MMU;

impl CPU {
    pub fn push(&mut self, opcode: u16, mmu: &mut MMU) {
        self.registers.sp -= 2;
        
        let value = match opcode {
            0xC5 => self.registers.get_bc(),
            0xD5 => self.registers.get_de(),
            0xE5 => self.registers.get_hl(),
            0xF5 => self.registers.get_af(),
            _ => panic!("You should not be here push"),
        };

        mmu.write_word(self.registers.sp, value);
        self.clock += 16;
    }
}