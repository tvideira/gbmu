use super::super::CPU;

impl CPU {
    pub fn nop(&mut self) {
        self.clock += 4
    }
}