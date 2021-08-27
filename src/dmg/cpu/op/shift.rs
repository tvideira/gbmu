use super::super::CPU;
use super::super::MMU;

impl CPU {
    pub fn sla_r(&mut self, opcode: u16) {
        let value: u8;
        match opcode {
            0xCB20 => { value = self.registers.b; self.registers.b = self.registers.b << 1; },
            0xCB21 => { value = self.registers.c; self.registers.c = self.registers.c << 1; },
            0xCB22 => { value = self.registers.d; self.registers.d = self.registers.d << 1; },
            0xCB23 => { value = self.registers.e; self.registers.e = self.registers.e << 1; },
            0xCB24 => { value = self.registers.h; self.registers.h = self.registers.h << 1; },
            0xCB25 => { value = self.registers.l; self.registers.l = self.registers.l << 1; },
            0xCB27 => { value = self.registers.a; self.registers.a = self.registers.a << 1; },
            _ => panic!("You should not be here sla r"),
        };
        
        self.registers.set_z_flag((value << 1) == 0);
        self.registers.set_n_flag(false);
        self.registers.set_h_flag(false);
        self.registers.set_c_flag((value >> 7) == 0x01); 
        self.clock += 8;
    }

    pub fn srl_r(&mut self, opcode: u16) {
        let value: u8;
        match opcode {
            0xCB38 => { value = self.registers.b; self.registers.b = self.registers.b >> 1; },
            0xCB39 => { value = self.registers.c; self.registers.c = self.registers.c >> 1; },
            0xCB3A => { value = self.registers.d; self.registers.d = self.registers.d >> 1; },
            0xCB3B => { value = self.registers.e; self.registers.e = self.registers.e >> 1; },
            0xCB3C => { value = self.registers.h; self.registers.h = self.registers.h >> 1; },
            0xCB3D => { value = self.registers.l; self.registers.l = self.registers.l >> 1; },
            0xCB3F => { value = self.registers.a; self.registers.a = self.registers.a >> 1; },
            _ => panic!("You should not be here srl r"),
        };
        
        self.registers.set_z_flag((value >> 1) == 0);
        self.registers.set_n_flag(false);
        self.registers.set_h_flag(false);
        self.registers.set_c_flag((value & 0x01) == 0x01); 
        self.clock += 8;
    }
}