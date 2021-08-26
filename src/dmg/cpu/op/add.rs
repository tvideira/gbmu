use super::super::CPU;
use super::super::MMU;

impl CPU {
    pub fn add_rr(&mut self, opcode: u16, mmu: & MMU) {
        let a = self.registers.a;
        let value = match opcode {
            0x86 => mmu.read_byte(self.registers.get_hl()),
            0xC6 => { self.registers.pc += 1; mmu.read_byte(self.registers.pc - 1) },
            _ => panic!("You should not be here add rr"),

        };

        self.registers.a = a.wrapping_add(value);

        self.registers.set_h_flag(((a & 0x0F) + (value & 0x0F)) > 0x0F);
        self.registers.set_c_flag(self.registers.a < a);
        self.registers.set_n_flag(false);
        self.registers.set_z_flag(self.registers.a == 0);

        self.clock += 8;
    }
}