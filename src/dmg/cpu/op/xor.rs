use super::super::CPU;
use super::super::MMU;

impl CPU {
    pub fn xor_r(&mut self, opcode: u16) {
        let value = match opcode & 0xF {
            0x8 => self.registers.b,
            0x9 => self.registers.c,
            0xA => self.registers.d,
            0xB => self.registers.e,
            0xC => self.registers.h,
            0xD => self.registers.l,
            0xF => self.registers.a,
            _ => panic!("you should not be here xor r"),
        };

        self.registers.a ^= value;

        self.registers.set_z_flag(self.registers.a == 0);
        self.registers.set_n_flag(false);
        self.registers.set_h_flag(false);
        self.registers.set_c_flag(false);

        self.clock += 4;
    }

    pub fn xor_rr(&mut self, opcode: u16, mmu: & MMU) {
        let value = match opcode {
            0xAE => mmu.read_byte(self.registers.get_hl()),
            0xEE => { self.registers.pc += 1; mmu.read_byte(self.registers.pc - 1) },
            _ => panic!("You should not be here add rr"),

        };

        self.registers.a ^= value;

        self.registers.set_z_flag(self.registers.a == 0);
        self.registers.set_n_flag(false);
        self.registers.set_h_flag(false);
        self.registers.set_c_flag(false);

        self.clock += 8;
    }
}