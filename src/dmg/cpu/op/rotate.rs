use super::super::CPU;
use super::super::MMU;

impl CPU {
    pub fn rlca(&mut self) {
        let value = self.registers.a;

        self.registers.a = value << 1 | value >> 7;
        self.registers.set_z_flag(false);
        self.registers.set_n_flag(false);
        self.registers.set_h_flag(false);
        self.registers.set_c_flag((value >> 7) == 1);

        self.clock += 4;
    }

    pub fn rrca(&mut self) {
        let value = self.registers.a;

        self.registers.a = value >> 1 | value << 7;

        self.registers.set_z_flag(false);
        self.registers.set_n_flag(false);
        self.registers.set_h_flag(false);
        self.registers.set_c_flag((value & 0x01) == 0x01);

        self.clock += 4;
    }

    pub fn rla(&mut self) {
        let value = self.registers.a;
        let c_flag = self.registers.get_c_flag() as u8;

        self.registers.a = value << 1 | c_flag;

        self.registers.set_z_flag(false);
        self.registers.set_n_flag(false);
        self.registers.set_h_flag(false);
        self.registers.set_c_flag((value >> 7) == 1);

        self.clock += 4;
    }

    pub fn rra(&mut self) {
        let value = self.registers.a;
        let c_flag = self.registers.get_c_flag() as u8;

        self.registers.a = value >> 1 | c_flag << 7;

        self.registers.set_z_flag(false);
        self.registers.set_n_flag(false);
        self.registers.set_h_flag(false);
        self.registers.set_c_flag((value & 0x01) == 0x01);

        self.clock += 4;
    }

    pub fn rl_r(&mut self, opcode: u16) {
        let value: u8;
        let c_flag = self.registers.get_c_flag() as u8;
        match opcode {
            0xCB10 => { value = self.registers.b; self.registers.b = value << 1 | c_flag; },
            0xCB11 => { value = self.registers.c; self.registers.c = value << 1 | c_flag; },
            0xCB12 => { value = self.registers.d; self.registers.d = value << 1 | c_flag; },
            0xCB13 => { value = self.registers.e; self.registers.e = value << 1 | c_flag; },
            0xCB14 => { value = self.registers.h; self.registers.h = value << 1 | c_flag; },
            0xCB15 => { value = self.registers.l; self.registers.l = value << 1 | c_flag; },
            0xCB17 => { value = self.registers.a; self.registers.a = value << 1 | c_flag; },
            _ => panic!("You should not be here rl r"),
        };

        self.registers.set_z_flag((value << 1 | c_flag) == 0);

        self.registers.set_n_flag(false);
        self.registers.set_h_flag(false);
        self.registers.set_c_flag((value >> 7) == 0x01);

        self.clock += 8;
    }

    pub fn rr_r(&mut self, opcode: u16) {
        let value: u8;
        let c_flag = self.registers.get_c_flag() as u8;
        match opcode {
            0xCB18 => { value = self.registers.b; self.registers.b = value >> 1 | c_flag << 7; },
            0xCB19 => { value = self.registers.c; self.registers.c = value >> 1 | c_flag << 7; },
            0xCB1A => { value = self.registers.d; self.registers.d = value >> 1 | c_flag << 7; },
            0xCB1B => { value = self.registers.e; self.registers.e = value >> 1 | c_flag << 7; },
            0xCB1C => { value = self.registers.h; self.registers.h = value >> 1 | c_flag << 7; },
            0xCB1D => { value = self.registers.l; self.registers.l = value >> 1 | c_flag << 7; },
            0xCB1F => { value = self.registers.a; self.registers.a = value >> 1 | c_flag << 7; },
            _ => panic!("You should not be here rr r"),
        };

        self.registers.set_z_flag((value >> 1 | c_flag << 7) == 0);

        self.registers.set_n_flag(false);
        self.registers.set_h_flag(false);
        self.registers.set_c_flag((value & 0x01) == 0x01);

        self.clock += 8;
    }
}