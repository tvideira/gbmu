pub mod add;
pub mod and;
pub mod bit;
pub mod call;
pub mod cp;
pub mod dec;
pub mod inc;
pub mod interrupt;
pub mod jump;
pub mod ld;
pub mod misc;
pub mod or;
pub mod pop;
pub mod push;
pub mod res;
pub mod ret;
pub mod rotate;
pub mod shift;
pub mod sub;
pub mod swap;
pub mod xor;

// R  = REGISTER A, B, C, D, F, H, L
// RR = REGISTER AF, BC, DE, HL, SP, PC
// N  = NEXT BYTE
// NN = NEXT 2 BYTES
// A  = REGISTER A
// B  = BIT
// HL = REGISTER HL
// HLI= REGISTER HL THEN INCREMENT REGISTER HL
// HLD= REGISTER HL THEN DECREMENT REGISTER HL
// ION= I/O REGISTERS (ADDRESS FF00) + N
// IOC= I/O REGISTERS (ADDRESS FF00) + REGISTER C

#[allow(non_camel_case_types)]
pub enum Op {
    // NOP
    NOP,
    
    // LD
    LD_RR_NN,
    LD_R_N,

    LD_NN_SP,

    LD_A_RR,
    LD_A_NN,

    LD_HLI_A,
    LD_A_HLI,
    LD_HLD_A,
    LD_A_HLD,

    LD_RR_A,
    LD_NN_A,

    LD_R_R,
    LD_R_HL,

    LD_HL_R,
    LD_HL_N,

    LD_ION_A,
    LD_A_ION,
    LD_IOC_A,
    LD_A_IOC,

    LD_HL_SP_N, // N IS SIGNED
    LD_SP_HL,

    // JUMP
    JR_N, // N IS SIGNED
    JP_NN,
    JP_HL,

    // CPL
    CPL,

    // DAA
    DAA,

    // RST
    RST,

    // INC
    INC_R,
    INC_RR,
    INC_HL,

    // DEC
    DEC_R,
    DEC_RR,
    DEC_HL,

    // ADD
    ADD_R,
    ADD_RR,
    ADD_HL_RR,
    ADD_SP_N,
    ADC_R,
    ADC_RR,

    // SUB
    SUB_R,
    SUB_RR,

    // OR
    OR_R,
    OR_RR,

    // AND
    AND_R,
    AND_RR,

    // XOR
    XOR_R,
    XOR_RR,

    // CP
    CP_R,
    CP_RR,

    // POP
    POP,

    // PUSH
    PUSH,

    // RET
    RET,

    // CALL
    CALL,

    // ROTATE
    RLCA,
    RLA,
    RRA,
    RRCA,

    // HALT + STOP
    STOP,
    HALT,

    // INTERRUPTS
    DI,
    EI,
    RETI,

    // PREFIX CB
    // ROTATE
    RL_R,
    RR_R,
    RL_HL,
    RR_HL,

    //SHIFT
    SLA_R,
    SLA_HL,
    SRL_R,
    SRL_HL,

    // SWAP
    SWAP_R,
    SWAP_HL,

    // BIT
    BIT_B_R,
    BIT_B_HL,

    // RES
    RES_B_R,
    RES_B_HL,
}

impl Op {
    pub fn decode(opcode: u16, addr: u16) -> Self {
        match opcode {
            0x00 => Self::NOP,

            0x01 => Self::LD_RR_NN, // BC
            0x11 => Self::LD_RR_NN, // DE
            0x21 => Self::LD_RR_NN, // HL
            0x31 => Self::LD_RR_NN, // SP

            0x02 => Self::LD_RR_A, // (BC)
            0x12 => Self::LD_RR_A, // (DE)
            0x77 => Self::LD_RR_A, // (HL)

            0x03 => Self::INC_RR, // BC
            0x13 => Self::INC_RR, // DE
            0x23 => Self::INC_RR, // HL
            0x33 => Self::INC_RR, // SP

            0x04 => Self::INC_R,  // B
            0x0C => Self::INC_R,  // C
            0x14 => Self::INC_R,  // D
            0x1C => Self::INC_R,  // E
            0x24 => Self::INC_R,  // H
            0x2C => Self::INC_R,  // L
            0x34 => Self::INC_HL, // (HL)
            0x3C => Self::INC_R,  // A

            0x05 => Self::DEC_R,  // B
            0x0D => Self::DEC_R,  // C
            0x15 => Self::DEC_R,  // D
            0x1D => Self::DEC_R,  // E
            0x25 => Self::DEC_R,  // H
            0x2D => Self::DEC_R,  // L
            0x35 => Self::DEC_HL, // (HL)
            0x3D => Self::DEC_R,  // A

            0x06 => Self::LD_R_N, // B
            0x0E => Self::LD_R_N, // C
            0x16 => Self::LD_R_N, // D
            0x1E => Self::LD_R_N, // E
            0x26 => Self::LD_R_N, // H
            0x2E => Self::LD_R_N, // L
            0x3E => Self::LD_R_N, // A

            0x07 => Self::RLCA,
            0x0F => Self::RRCA,
            0x17 => Self::RLA,
            0x1F => Self::RRA,

            0x08 => Self::LD_NN_SP,

            0x09 => Self::ADD_HL_RR, // BC
            0x19 => Self::ADD_HL_RR, // DE
            0x29 => Self::ADD_HL_RR, // HL
            0x39 => Self::ADD_HL_RR, // SP

            0x0A => Self::LD_A_RR, // (BC)
            0x1A => Self::LD_A_RR, // (DE)

            0x0B => Self::DEC_RR, // BC
            0x1B => Self::DEC_RR, // DE
            0x2B => Self::DEC_RR, // HL
            0x3B => Self::DEC_RR, // SP

            0x10 => Self::STOP,

            0x18 => Self::JR_N, // NO CONDITION
            0x20 => Self::JR_N, // FLAG Z = 0
            0x28 => Self::JR_N, // FLAG Z = 1
            0x30 => Self::JR_N, // FLAG C = 0
            0x38 => Self::JR_N, // FLAG C = 1


            0x22 => Self::LD_HLI_A,
            0x2A => Self::LD_A_HLI,

            0x27 => Self::DAA,
            0x2F => Self::CPL,

            0x32 => Self::LD_HLD_A,
            0x3A => Self::LD_A_HLD,

            0x36 => Self::LD_HL_N,
            
            0x40 => Self::LD_R_R, // B, B
            0x41 => Self::LD_R_R, // B, C
            0x42 => Self::LD_R_R, // B, D
            0x43 => Self::LD_R_R, // B, E
            0x44 => Self::LD_R_R, // B, H
            0x45 => Self::LD_R_R, // B, L
            0x47 => Self::LD_R_R, // B, A

            0x48 => Self::LD_R_R, // C, B
            0x49 => Self::LD_R_R, // C, C
            0x4A => Self::LD_R_R, // C, D
            0x4B => Self::LD_R_R, // C, E
            0x4C => Self::LD_R_R, // C, H
            0x4D => Self::LD_R_R, // C, L
            0x4F => Self::LD_R_R, // C, A

            0x50 => Self::LD_R_R, // D, B
            0x51 => Self::LD_R_R, // D, C
            0x52 => Self::LD_R_R, // D, D
            0x53 => Self::LD_R_R, // D, E
            0x54 => Self::LD_R_R, // D, H
            0x55 => Self::LD_R_R, // D, L
            0x57 => Self::LD_R_R, // D, A

            0x58 => Self::LD_R_R, // E, B
            0x59 => Self::LD_R_R, // E, C
            0x5A => Self::LD_R_R, // E, D
            0x5B => Self::LD_R_R, // E, E
            0x5C => Self::LD_R_R, // E, H
            0x5D => Self::LD_R_R, // E, L
            0x5F => Self::LD_R_R, // E, A

            0x60 => Self::LD_R_R, // H, B
            0x61 => Self::LD_R_R, // H, C
            0x62 => Self::LD_R_R, // H, D
            0x63 => Self::LD_R_R, // H, E
            0x64 => Self::LD_R_R, // H, H
            0x65 => Self::LD_R_R, // H, L
            0x67 => Self::LD_R_R, // H, A

            0x68 => Self::LD_R_R, // L, B
            0x69 => Self::LD_R_R, // L, C
            0x6A => Self::LD_R_R, // L, D
            0x6B => Self::LD_R_R, // L, E
            0x6C => Self::LD_R_R, // L, H
            0x6D => Self::LD_R_R, // L, L
            0x6F => Self::LD_R_R, // L, A

            0x70 => Self::LD_HL_R, // (HL), B
            0x71 => Self::LD_HL_R, // (HL), C
            0x72 => Self::LD_HL_R, // (HL), D
            0x73 => Self::LD_HL_R, // (HL), E
            0x74 => Self::LD_HL_R, // (HL), H
            0x75 => Self::LD_HL_R, // (HL), L

            0x78 => Self::LD_R_R, // A, B
            0x79 => Self::LD_R_R, // A, C
            0x7A => Self::LD_R_R, // A, D
            0x7B => Self::LD_R_R, // A, E
            0x7C => Self::LD_R_R, // A, H
            0x7D => Self::LD_R_R, // A, L
            0x7F => Self::LD_R_R, // A, A

            0x46 => Self::LD_R_HL, // B, (HL)
            0x4E => Self::LD_R_HL, // C, (HL)
            0x56 => Self::LD_R_HL, // D, (HL)
            0x5E => Self::LD_R_HL, // E, (HL)
            0x66 => Self::LD_R_HL, // H, (HL)
            0x6E => Self::LD_R_HL, // L, (HL)
            0x7E => Self::LD_R_HL, // A, (HL)

            0x76 => Self::HALT,

            0x80 => Self::ADD_R, // A, B
            0x81 => Self::ADD_R, // A, C
            0x82 => Self::ADD_R, // A, D
            0x83 => Self::ADD_R, // A, E
            0x84 => Self::ADD_R, // A, H
            0x85 => Self::ADD_R, // A, L
            0x87 => Self::ADD_R, // A, A

            0x86 => Self::ADD_RR, // (HL)
            0xC6 => Self::ADD_RR, // (PC)

            0x88 => Self::ADC_R, // A, B
            0x89 => Self::ADC_R, // A, C
            0x8A => Self::ADC_R, // A, D
            0x8B => Self::ADC_R, // A, E
            0x8C => Self::ADC_R, // A, H
            0x8D => Self::ADC_R, // A, L
            0x8F => Self::ADC_R, // A, A

            0x8E => Self::ADC_RR, // (HL)
            0xCE => Self::ADC_RR, // (PC)

            0x90 => Self::SUB_R, // B
            0x91 => Self::SUB_R, // C
            0x92 => Self::SUB_R, // D
            0x93 => Self::SUB_R, // E
            0x94 => Self::SUB_R, // H
            0x95 => Self::SUB_R, // L
            0x97 => Self::SUB_R, // A

            0x96 => Self::SUB_RR, // (HL)
            0xD6 => Self::SUB_RR, // (PC)

            0xA0 => Self::AND_R, // B
            0xA1 => Self::AND_R, // C
            0xA2 => Self::AND_R, // D
            0xA3 => Self::AND_R, // E
            0xA4 => Self::AND_R, // H
            0xA5 => Self::AND_R, // L
            0xA7 => Self::AND_R, // A

            0xA6 => Self::AND_RR, // (HL)
            0xE6 => Self::AND_RR, // (PC)

            0xA8 => Self::XOR_R, // B
            0xA9 => Self::XOR_R, // C
            0xAA => Self::XOR_R, // D
            0xAB => Self::XOR_R, // E
            0xAC => Self::XOR_R, // H
            0xAD => Self::XOR_R, // L
            0xAF => Self::XOR_R, // A

            0xAE => Self::XOR_RR, // (HL)
            0xEE => Self::XOR_RR, // (PC)

            0xB0 => Self::OR_R, // B
            0xB1 => Self::OR_R, // C
            0xB2 => Self::OR_R, // D
            0xB3 => Self::OR_R, // E
            0xB4 => Self::OR_R, // H
            0xB5 => Self::OR_R, // L
            0xB7 => Self::OR_R, // A

            0xB6 => Self::OR_RR, // (HL)
            0xF6 => Self::OR_RR, // (PC)

            0xB8 => Self::CP_R, // B
            0xB9 => Self::CP_R, // C
            0xBA => Self::CP_R, // D
            0xBB => Self::CP_R, // E
            0xBC => Self::CP_R, // H
            0xBD => Self::CP_R, // L
            0xBF => Self::CP_R, // A

            0xBE => Self::CP_RR, // (HL)
            0xFE => Self::CP_RR, // (PC)

            0xC1 => Self::POP, // BC
            0xD1 => Self::POP, // DE
            0xE1 => Self::POP, // HL
            0xF1 => Self::POP, // AF

            0xC3 => Self::JP_NN, // NO CONDITION
            0xC2 => Self::JP_NN, // FLAG Z = 0
            0xCA => Self::JP_NN, // FLAG Z = 1
            0xD2 => Self::JP_NN, // FLAG C = 0
            0xDA => Self::JP_NN, // FLAG C = 1

            0xC5 => Self::PUSH, // BC
            0xD5 => Self::PUSH, // DE
            0xE5 => Self::PUSH, // FL
            0xF5 => Self::PUSH, // AF

            0xC7 => Self::RST, // 0x00
            0xCF => Self::RST, // 0x08
            0xD7 => Self::RST, // 0x10
            0xDF => Self::RST, // 0x18
            0xE7 => Self::RST, // 0x20
            0xEF => Self::RST, // 0x28
            0xF7 => Self::RST, // 0x30
            0xFF => Self::RST, // 0x38

            0xC9 => Self::RET, // NO CONDITION
            0xC0 => Self::RET, // FLAG Z = 0
            0xC8 => Self::RET, // FLAG Z = 1
            0xD0 => Self::RET, // FLAG C = 0
            0xD8 => Self::RET, // FLAG C = 1

            0xCD => Self::CALL, // NO CONDITION
            0xC4 => Self::CALL, // FLAG Z = 0
            0xCC => Self::CALL, // FLAG Z = 1
            0xD4 => Self::CALL, // FLAG C = 0
            0xDC => Self::CALL, // FLAG C = 1

            0xD9 => Self::RETI,

            0xE0 => Self::LD_ION_A,
            0xF0 => Self::LD_A_ION,

            0xE8 => Self::ADD_SP_N,

            0xE9 => Self::JP_HL,

            0xE2 => Self::LD_IOC_A,
            0xF2 => Self::LD_A_IOC,

            0xEA => Self::LD_NN_A,
            0xFA => Self::LD_A_NN,

            0xF3 => Self::DI,
            0xFB => Self::EI,

            0xF8 => Self::LD_HL_SP_N, // N IS SIGNED
            0xF9 => Self::LD_SP_HL,

            
            // PREFIX CB
            0xCB10 => Self::RL_R, // B
            0xCB11 => Self::RL_R, // C
            0xCB12 => Self::RL_R, // D
            0xCB13 => Self::RL_R, // E
            0xCB14 => Self::RL_R, // H
            0xCB15 => Self::RL_R, // L
            0xCB17 => Self::RL_R, // A

            0xCB18 => Self::RR_R, // B
            0xCB19 => Self::RR_R, // C
            0xCB1A => Self::RR_R, // D
            0xCB1B => Self::RR_R, // E
            0xCB1C => Self::RR_R, // H
            0xCB1D => Self::RR_R, // L
            0xCB1F => Self::RR_R, // A

            0xCB20 => Self::SLA_R, // B
            0xCB21 => Self::SLA_R, // C
            0xCB22 => Self::SLA_R, // D
            0xCB23 => Self::SLA_R, // E
            0xCB24 => Self::SLA_R, // H
            0xCB25 => Self::SLA_R, // L
            0xCB27 => Self::SLA_R, // A

            0xCB30 => Self::SWAP_R, // B
            0xCB31 => Self::SWAP_R, // C
            0xCB32 => Self::SWAP_R, // D
            0xCB33 => Self::SWAP_R, // E
            0xCB34 => Self::SWAP_R, // H
            0xCB35 => Self::SWAP_R, // L
            0xCB37 => Self::SWAP_R, // A

            0xCB38 => Self::SRL_R, // B
            0xCB39 => Self::SRL_R, // C
            0xCB3A => Self::SRL_R, // D
            0xCB3B => Self::SRL_R, // E
            0xCB3C => Self::SRL_R, // H
            0xCB3D => Self::SRL_R, // L
            0xCB3F => Self::SRL_R, // A

            0xCB40 => Self::BIT_B_R, // 0, B
            0xCB41 => Self::BIT_B_R, // 0, C
            0xCB42 => Self::BIT_B_R, // 0, D
            0xCB43 => Self::BIT_B_R, // 0, E
            0xCB44 => Self::BIT_B_R, // 0, H
            0xCB45 => Self::BIT_B_R, // 0, L
            0xCB47 => Self::BIT_B_R, // 0, A

            0xCB48 => Self::BIT_B_R, // 1, B
            0xCB49 => Self::BIT_B_R, // 1, C
            0xCB4A => Self::BIT_B_R, // 1, D
            0xCB4B => Self::BIT_B_R, // 1, E
            0xCB4C => Self::BIT_B_R, // 1, H
            0xCB4D => Self::BIT_B_R, // 1, L
            0xCB4F => Self::BIT_B_R, // 1, A

            0xCB50 => Self::BIT_B_R, // 2, B
            0xCB51 => Self::BIT_B_R, // 2, C
            0xCB52 => Self::BIT_B_R, // 2, D
            0xCB53 => Self::BIT_B_R, // 2, E
            0xCB54 => Self::BIT_B_R, // 2, H
            0xCB55 => Self::BIT_B_R, // 2, L
            0xCB57 => Self::BIT_B_R, // 2, A

            0xCB58 => Self::BIT_B_R, // 3, B
            0xCB59 => Self::BIT_B_R, // 3, C
            0xCB5A => Self::BIT_B_R, // 3, D
            0xCB5B => Self::BIT_B_R, // 3, E
            0xCB5C => Self::BIT_B_R, // 3, H
            0xCB5D => Self::BIT_B_R, // 3, L
            0xCB5F => Self::BIT_B_R, // 3, A

            0xCB60 => Self::BIT_B_R, // 4, B
            0xCB61 => Self::BIT_B_R, // 4, C
            0xCB62 => Self::BIT_B_R, // 4, D
            0xCB63 => Self::BIT_B_R, // 4, E
            0xCB64 => Self::BIT_B_R, // 4, H
            0xCB65 => Self::BIT_B_R, // 4, L
            0xCB67 => Self::BIT_B_R, // 4, A

            0xCB68 => Self::BIT_B_R, // 5, B
            0xCB69 => Self::BIT_B_R, // 5, C
            0xCB6A => Self::BIT_B_R, // 5, D
            0xCB6B => Self::BIT_B_R, // 5, E
            0xCB6C => Self::BIT_B_R, // 5, H
            0xCB6D => Self::BIT_B_R, // 5, L
            0xCB6F => Self::BIT_B_R, // 5, A

            0xCB70 => Self::BIT_B_R, // 6, B
            0xCB71 => Self::BIT_B_R, // 6, C
            0xCB72 => Self::BIT_B_R, // 6, D
            0xCB73 => Self::BIT_B_R, // 6, E
            0xCB74 => Self::BIT_B_R, // 6, H
            0xCB75 => Self::BIT_B_R, // 6, L
            0xCB77 => Self::BIT_B_R, // 6, A

            0xCB78 => Self::BIT_B_R, // 7, B
            0xCB79 => Self::BIT_B_R, // 7, C
            0xCB7A => Self::BIT_B_R, // 7, D
            0xCB7B => Self::BIT_B_R, // 7, E
            0xCB7C => Self::BIT_B_R, // 7, H
            0xCB7D => Self::BIT_B_R, // 7, L
            0xCB7F => Self::BIT_B_R, // 7, A

            0xCB46 => Self::BIT_B_HL, // 0, (HL)
            0xCB4E => Self::BIT_B_HL, // 1, (HL)
            0xCB56 => Self::BIT_B_HL, // 2, (HL)
            0xCB5E => Self::BIT_B_HL, // 3, (HL)
            0xCB66 => Self::BIT_B_HL, // 4, (HL)
            0xCB6E => Self::BIT_B_HL, // 5, (HL)
            0xCB76 => Self::BIT_B_HL, // 6, (HL)
            0xCB7E => Self::BIT_B_HL, // 7, (HL)

            0xCB80 => Self::RES_B_R, // 0, B
            0xCB81 => Self::RES_B_R, // 0, C
            0xCB82 => Self::RES_B_R, // 0, D
            0xCB83 => Self::RES_B_R, // 0, E
            0xCB84 => Self::RES_B_R, // 0, H
            0xCB85 => Self::RES_B_R, // 0, L
            0xCB87 => Self::RES_B_R, // 0, A

            0xCB88 => Self::RES_B_R, // 1, B
            0xCB89 => Self::RES_B_R, // 1, C
            0xCB8A => Self::RES_B_R, // 1, D
            0xCB8B => Self::RES_B_R, // 1, E
            0xCB8C => Self::RES_B_R, // 1, H
            0xCB8D => Self::RES_B_R, // 1, L
            0xCB8F => Self::RES_B_R, // 1, A

            0xCB90 => Self::RES_B_R, // 2, B
            0xCB91 => Self::RES_B_R, // 2, C
            0xCB92 => Self::RES_B_R, // 2, D
            0xCB93 => Self::RES_B_R, // 2, E
            0xCB94 => Self::RES_B_R, // 2, H
            0xCB95 => Self::RES_B_R, // 2, L
            0xCB97 => Self::RES_B_R, // 2, A

            0xCB98 => Self::RES_B_R, // 3, B
            0xCB99 => Self::RES_B_R, // 3, C
            0xCB9A => Self::RES_B_R, // 3, D
            0xCB9B => Self::RES_B_R, // 3, E
            0xCB9C => Self::RES_B_R, // 3, H
            0xCB9D => Self::RES_B_R, // 3, L
            0xCB9F => Self::RES_B_R, // 3, A

            0xCBA0 => Self::RES_B_R, // 4, B
            0xCBA1 => Self::RES_B_R, // 4, C
            0xCBA2 => Self::RES_B_R, // 4, D
            0xCBA3 => Self::RES_B_R, // 4, E
            0xCBA4 => Self::RES_B_R, // 4, H
            0xCBA5 => Self::RES_B_R, // 4, L
            0xCBA7 => Self::RES_B_R, // 4, A

            0xCBA8 => Self::RES_B_R, // 5, B
            0xCBA9 => Self::RES_B_R, // 5, C
            0xCBAA => Self::RES_B_R, // 5, D
            0xCBAB => Self::RES_B_R, // 5, E
            0xCBAC => Self::RES_B_R, // 5, H
            0xCBAD => Self::RES_B_R, // 5, L
            0xCBAF => Self::RES_B_R, // 5, A

            0xCBB0 => Self::RES_B_R, // 6, B
            0xCBB1 => Self::RES_B_R, // 6, C
            0xCBB2 => Self::RES_B_R, // 6, D
            0xCBB3 => Self::RES_B_R, // 6, E
            0xCBB4 => Self::RES_B_R, // 6, H
            0xCBB5 => Self::RES_B_R, // 6, L
            0xCBB7 => Self::RES_B_R, // 6, A

            0xCBB8 => Self::RES_B_R, // 7, B
            0xCBB9 => Self::RES_B_R, // 7, C
            0xCBBA => Self::RES_B_R, // 7, D
            0xCBBB => Self::RES_B_R, // 7, E
            0xCBBC => Self::RES_B_R, // 7, H
            0xCBBD => Self::RES_B_R, // 7, L
            0xCBBF => Self::RES_B_R, // 7, A

            0xCB86 => Self::RES_B_HL, // 0, (HL)
            0xCB8E => Self::RES_B_HL, // 1, (HL)
            0xCB96 => Self::RES_B_HL, // 2, (HL)
            0xCB9E => Self::RES_B_HL, // 3, (HL)
            0xCBA6 => Self::RES_B_HL, // 4, (HL)
            0xCBAE => Self::RES_B_HL, // 5, (HL)
            0xCBB6 => Self::RES_B_HL, // 6, (HL)
            0xCBBE => Self::RES_B_HL, // 7, (HL)

        _ => panic!("OPCODE {:X} at 0x{:04X} NOT IMPLEMENTED YET", opcode, addr),
        }
    }
}