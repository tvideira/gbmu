mod registers;
mod op;

use registers::REGISTERS;
use op::*;

use super::MMU;

// CPU STRUCT
#[derive(Default)]
pub struct CPU {
    registers: REGISTERS,
    clock: u32,
    ime: bool,
}

impl CPU {
    pub fn step(&mut self, mmu: &mut MMU) -> u32 {
        self.clock = 0;

        if self.registers.pc == 0x100 { println!("bios disabled"); mmu.disable_bios(); }

        // HANDLE INSTRUCTION
        let addr = self.registers.pc;
        let mut opcode = mmu.read_byte(addr) as u16;
        if opcode == 0xCB {
            opcode = opcode << 8 | (mmu.read_byte(addr + 1) as u16);
            self.registers.pc += 1;
        }
        self.registers.pc += 1;

        let op = Op::decode(opcode, addr);

        match op {
            Op::NOP => self.nop(),
            
            Op::LD_RR_NN    => self.ld_rr_nn(opcode, mmu),
            Op::LD_RR_A     => self.ld_rr_a(opcode, mmu),
            
            Op::INC_R       => self.inc_r(opcode),
            Op::INC_RR      => self.inc_rr(opcode),
            
            Op::DEC_R       => self.dec_r(opcode),
            Op::DEC_RR      => self.dec_rr(opcode),
            
            Op::LD_R_N      => self.ld_r_n(opcode, mmu),
            Op::LD_A_RR     => self.ld_a_rr(opcode, mmu),

            Op::RLA         => self.rla(),

            Op::JR_N        => self.jr_n(opcode, mmu),

            Op::LD_HLI_A    => self.ld_hli_a(mmu),
            Op::LD_A_HLI    => self.ld_a_hli(mmu),
            Op::LD_HLD_A    => self.ld_hld_a(mmu),
            Op::LD_A_HLD    => self.ld_a_hld(mmu),
            Op::LD_HL_N     => self.ld_hl_n(mmu),
            Op::LD_R_R      => self.ld_r_r(opcode),

            Op::ADD_RR      => self.add_rr(opcode, mmu),

            Op::SUB_R       => self.sub_r(opcode),
            Op::XOR_R       => self.xor_r(opcode),
            Op::OR_R        => self.or_r(opcode),

            Op::CP_RR       => self.cp_rr(opcode, mmu),

            Op::POP         => self.pop(opcode, mmu),
            Op::PUSH        => self.push(opcode, mmu),

            Op::JP_NN       => self.jp_nn(opcode, mmu),
            
            Op::RET         => self.ret(opcode, mmu),
            Op::CALL        => self.call(opcode, mmu),

            Op::LD_ION_A    => self.ld_ion_a(mmu),
            Op::LD_A_ION    => self.ld_a_ion(mmu),
            Op::LD_IOC_A    => self.ld_ioc_a(mmu),
            Op::LD_A_IOC    => self.ld_a_ioc(mmu),

            Op::DI          => self.di(),

            Op::LD_A_NN     => self.ld_a_nn(mmu),

            Op::RL_R        => self.rl_r(opcode),
            Op::BIT_B_R     => self.bit_b_r(opcode),

            _ => panic!("INSTRUCTION {:X} at 0x{:04X} NOT IMPLEMENTED YET", opcode, addr),
        }

        return self.clock;
    }
}