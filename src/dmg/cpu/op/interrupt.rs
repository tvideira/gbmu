use super::super::CPU;
use super::super::MMU;

impl CPU {
    pub fn di(&mut self) {
        self.interrupt_master_enable = false;
        self.clock += 4;
    }

    pub fn ei(&mut self) {
        self.interrupt_master_enable = true;
        self.clock += 4;
    }

    pub fn reti(&mut self, mmu: & MMU) {
        // POP TWO BYTES AND JUMP TO ADDRESS
        self.registers.pc = mmu.read_word(self.registers.sp);
        self.registers.sp += 2;

        self.interrupt_master_enable = true;
        self.clock += 16;
    }
}