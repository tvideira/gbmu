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
    interrupt_master_enable: bool,
    is_halted: bool,
    is_halted_until_button_pressed: bool,
}

impl CPU {
    pub fn step(&mut self, mmu: &mut MMU) -> u32 {
        self.clock = 0;

        // HANDLE INTERRUPTS
        self.handle_interrupts(mmu);

        // HANDLE INSTRUCTION
        if !self.is_halted {
            self.do_operation(mmu);
        } else {
            self.nop();
        }

        //HANDLE BIOS
        mmu.write_byte(0xFF00, 0xFF);
        if self.registers.pc == 0x100 { println!("bios disabled"); mmu.disable_bios(); }

        return self.clock;
    }

    pub fn handle_interrupts(&mut self, mmu: &mut MMU) {
        if !self.interrupt_master_enable {
            return ;
        }

        self.is_halted = false;

        let interrupt_enable = mmu.read_byte(0xFFFF);
        let interrupt_flag = mmu.read_byte(0xFF0F);
        let interrupt_mask = interrupt_enable & interrupt_flag & 0x1F;

        for i in 0..5 {
            if (interrupt_mask >> i) == 1 {
                mmu.write_byte(0xFF0F, interrupt_flag & (0xFF ^ (1 << i)));
                self.interrupt_master_enable = false;
                self.registers.sp -= 2;
                mmu.write_word(self.registers.sp, self.registers.pc);
                self.registers.pc = 0x40 + (0x08 * i);
                self.clock += 20;
                break ;
            }
        }

        self.clock += 20;
    }

    pub fn do_operation(&mut self, mmu: &mut MMU) {
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
            Op::LD_NN_SP    => self.ld_nn_sp(mmu),
            
            Op::INC_R       => self.inc_r(opcode),
            Op::INC_RR      => self.inc_rr(opcode),
            Op::INC_HL      => self.inc_hl(mmu),            
            Op::DEC_R       => self.dec_r(opcode),
            Op::DEC_RR      => self.dec_rr(opcode),
            Op::DEC_HL      => self.dec_hl(mmu),
            
            Op::LD_R_N      => self.ld_r_n(opcode, mmu),
            Op::LD_A_RR     => self.ld_a_rr(opcode, mmu),

            Op::RLCA        => self.rlca(),
            Op::RRCA        => self.rrca(),
            Op::RLA         => self.rla(),
            Op::RRA         => self.rra(),

            Op::JR_N        => self.jr_n(opcode, mmu),

            Op::LD_HLI_A    => self.ld_hli_a(mmu),
            Op::LD_A_HLI    => self.ld_a_hli(mmu),

            Op::DAA         => self.daa(),
            Op::CPL         => self.cpl(),

            Op::LD_HLD_A    => self.ld_hld_a(mmu),
            Op::LD_A_HLD    => self.ld_a_hld(mmu),
            
            Op::LD_HL_N     => self.ld_hl_n(mmu),
            Op::LD_R_R      => self.ld_r_r(opcode),
            Op::LD_R_HL     => self.ld_r_hl(opcode, mmu),
            Op::LD_HL_R     => self.ld_hl_r(opcode, mmu),

            //Op::STOP        => self.stop(),
            Op::HALT        => self.halt(),

            Op::ADD_R       => self.add_r(opcode),
            Op::ADD_RR      => self.add_rr(opcode, mmu),
            Op::ADD_HL_RR   => self.add_hl_rr(opcode),
            Op::ADC_R       => self.adc_r(opcode),
            Op::ADC_RR      => self.adc_rr(opcode, mmu),
            Op::SUB_R       => self.sub_r(opcode),            
            Op::SUB_RR      => self.sub_rr(opcode, mmu),            

            Op::AND_R       => self.and_r(opcode),
            Op::XOR_R       => self.xor_r(opcode),
            Op::OR_R        => self.or_r(opcode),
            Op::CP_R        => self.cp_r(opcode),

            Op::AND_RR      => self.and_rr(opcode, mmu),
            Op::XOR_RR      => self.xor_rr(opcode, mmu),
            Op::OR_RR       => self.or_rr(opcode, mmu),
            Op::CP_RR       => self.cp_rr(opcode, mmu),

            Op::POP         => self.pop(opcode, mmu),
            Op::PUSH        => self.push(opcode, mmu),

            Op::RST         => self.rst(opcode, mmu),

            Op::JP_NN       => self.jp_nn(opcode, mmu),
            Op::JP_HL       => self.jp_hl(),
            
            Op::RET         => self.ret(opcode, mmu),
            Op::CALL        => self.call(opcode, mmu),

            Op::LD_ION_A    => self.ld_ion_a(mmu),
            Op::LD_A_ION    => self.ld_a_ion(mmu),
            Op::LD_IOC_A    => self.ld_ioc_a(mmu),
            Op::LD_A_IOC    => self.ld_a_ioc(mmu),

            Op::RETI        => self.reti(mmu),
            Op::DI          => self.di(),
            Op::EI          => self.ei(),

            Op::LD_NN_A     => self.ld_nn_a(mmu),
            Op::LD_A_NN     => self.ld_a_nn(mmu),

            Op::LD_HL_SP_N  => self.ld_hl_sp_n(mmu),
            Op::LD_SP_HL    => self.ld_sp_hl(),

            Op::ADD_SP_N    => self.add_sp_n(mmu),

            Op::RL_R        => self.rl_r(opcode),
            Op::RR_R        => self.rr_r(opcode),
            Op::SLA_R       => self.sla_r(opcode),
            Op::SRL_R       => self.srl_r(opcode),
            Op::SWAP_R      => self.swap_r(opcode),
            Op::BIT_B_R     => self.bit_b_r(opcode),
            Op::BIT_B_HL    => self.bit_b_hl(opcode, mmu),
            Op::RES_B_R     => self.res_b_r(opcode),
            Op::RES_B_HL    => self.res_b_hl(opcode, mmu),

            _ => panic!("INSTRUCTION {:X} at 0x{:04X} NOT IMPLEMENTED YET", opcode, addr),
        }
    }
}