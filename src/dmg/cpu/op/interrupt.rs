use super::super::CPU;

impl CPU {
    pub fn di (&mut self) {
        self.ime = false;
        self.clock += 4;
    }
}