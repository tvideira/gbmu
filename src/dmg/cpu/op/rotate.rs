use super::super::CPU;
use super::super::MMU;

impl CPU {
    pub fn rla(&mut self) {
        let value = self.registers.a;
        let c_flag = self.registers.get_c_flag() as u8;
        self.registers.a = self.registers.a << 1 | c_flag;
        self.registers.set_c_flag((value >> 7) == 1); 
        self.registers.set_z_flag(false);
        self.registers.set_n_flag(false);
        self.registers.set_h_flag(false);
        self.clock += 4;
    }

    pub fn rl_r(&mut self, opcode: u16) {
        let value: u8;
        let c_flag = self.registers.get_c_flag() as u8;
        match opcode {
            0xCB10 => { value = self.registers.b; self.registers.b = self.registers.b << 1 | c_flag; },
            0xCB11 => { value = self.registers.c; self.registers.c = self.registers.c << 1 | c_flag; },
            0xCB12 => { value = self.registers.d; self.registers.d = self.registers.d << 1 | c_flag; },
            0xCB13 => { value = self.registers.e; self.registers.e = self.registers.e << 1 | c_flag; },
            0xCB14 => { value = self.registers.h; self.registers.h = self.registers.h << 1 | c_flag; },
            0xCB15 => { value = self.registers.l; self.registers.l = self.registers.l << 1 | c_flag; },
            0xCB17 => { value = self.registers.a; self.registers.a = self.registers.a << 1 | c_flag; },
            _ => panic!("You should not be here rl r"),
        };
        
        self.registers.set_c_flag((value >> 7) == 1); 
        self.registers.set_z_flag((value << 1 | c_flag) == 0);
        self.registers.set_n_flag(false);
        self.registers.set_h_flag(false);
        self.clock += 8;
    }
}