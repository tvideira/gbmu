use super::super::CPU;
use super::super::MMU;

impl CPU {
    pub fn sub_r(&mut self, opcode: u16) {
        let a = self.registers.a;
        let value = match opcode & 0xF {
            0x0 => self.registers.b,
            0x1 => self.registers.c,
            0x2 => self.registers.d,
            0x3 => self.registers.e,
            0x4 => self.registers.h,
            0x5 => self.registers.l,
            0x7 => self.registers.a,
            _ => panic!("you should not be here sub r"),
        };

        self.registers.a = a.wrapping_sub(value);

        self.registers.set_h_flag((value & 0x0F) > (a & 0x0F));
        self.registers.set_c_flag(value > a);
        self.registers.set_n_flag(true);
        self.registers.set_z_flag(self.registers.a == 0);

        self.clock += 4;
    }
}