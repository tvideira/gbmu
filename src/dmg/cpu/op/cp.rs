use super::super::CPU;
use super::super::MMU;

impl CPU {
    pub fn cp_rr(&mut self, opcode: u16, mmu: & MMU) {
        let a = self.registers.a;
        let value = match opcode {
            0xBE => mmu.read_byte(self.registers.get_hl()),
            0xFE => { self.registers.pc += 1; mmu.read_byte(self.registers.pc - 1) },
            _ => panic!("You should not be here cp rr"),

        };

        self.registers.set_z_flag(value == a);
        self.registers.set_n_flag(true);
        self.registers.set_h_flag((a & 0x0F) < (value & 0x0F));
        self.registers.set_c_flag(a < value);

        self.clock += 8;
    }
}