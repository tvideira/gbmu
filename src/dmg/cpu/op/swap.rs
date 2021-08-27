use super::super::CPU;
use super::super::MMU;

impl CPU {
    pub fn swap_r(&mut self, opcode: u16) {
        let value = match opcode {
            0xCB30 => { self.registers.b = self.registers.b << 4 | self.registers.b >> 4; self.registers.b },
            0xCB31 => { self.registers.c = self.registers.c << 4 | self.registers.c >> 4; self.registers.c },
            0xCB32 => { self.registers.d = self.registers.d << 4 | self.registers.d >> 4; self.registers.d },
            0xCB33 => { self.registers.e = self.registers.e << 4 | self.registers.e >> 4; self.registers.e },
            0xCB34 => { self.registers.h = self.registers.h << 4 | self.registers.h >> 4; self.registers.h },
            0xCB35 => { self.registers.l = self.registers.l << 4 | self.registers.l >> 4; self.registers.l },
            0xCB37 => { self.registers.a = self.registers.a << 4 | self.registers.a >> 4; self.registers.a },
            _ => panic!("You should not be here rl r"),
        };
        
        self.registers.set_z_flag(value == 0);
        self.registers.set_n_flag(false);
        self.registers.set_h_flag(false);
        self.registers.set_c_flag(false); 
        self.clock += 8;
    }
}