use crate::asm::*;
use crate::isa::*;
use crate::riscv::imm::Xlen;

// Helpers to assemble bit-fields
#[inline]
fn r_type(opcode: u32, rd: u8, funct3: u8, rs1: u8, rs2: u8, funct7: u8) -> u32 {
    ((funct7 as u32) << 25)
        | ((rs2 as u32) << 20)
        | ((rs1 as u32) << 15)
        | ((funct3 as u32) << 12)
        | ((rd as u32) << 7)
        | opcode
}

#[inline]
fn i_type(opcode: u32, rd: u8, funct3: u8, rs1: u8, imm12: u32) -> u32 {
    ((imm12 & 0xFFF) << 20)
        | ((rs1 as u32) << 15)
        | ((funct3 as u32) << 12)
        | ((rd as u32) << 7)
        | opcode
}

#[inline]
fn s_type(opcode: u32, funct3: u8, rs1: u8, rs2: u8, imm12: u32) -> u32 {
    let imm_low = (imm12 & 0x1F) as u32; // [4:0]
    let imm_high = ((imm12 >> 5) & 0x7F) as u32; // [11:5]
    (imm_high << 25)
        | ((rs2 as u32) << 20)
        | ((rs1 as u32) << 15)
        | ((funct3 as u32) << 12)
        | (imm_low << 7)
        | opcode
}

#[inline]
fn b_type(opcode: u32, funct3: u8, rs1: u8, rs2: u8, imm: u32) -> u32 {
    // imm layout in decode: [12][10:5][4:1][11]
    let bit_11_to_11 = ((imm >> 11) & 0x1) as u32; // to bit 7
    let bits_4_to_1 = ((imm >> 1) & 0xF) as u32; // to bits 11:8
    let bits_10_to_5 = ((imm >> 5) & 0x3F) as u32; // to bits 30:25
    let bit_12_to_12 = ((imm >> 12) & 0x1) as u32; // to bit 31

    (bit_12_to_12 << 31)
        | (bits_10_to_5 << 25)
        | ((rs2 as u32) << 20)
        | ((rs1 as u32) << 15)
        | ((funct3 as u32) << 12)
        | (bits_4_to_1 << 8)
        | (bit_11_to_11 << 7)
        | opcode
}

#[inline]
fn u_type(opcode: u32, rd: u8, imm_hi20: u32) -> u32 {
    (imm_hi20 & 0xFFFFF000) | ((rd as u32) << 7) | opcode
}

#[inline]
fn j_type(opcode: u32, rd: u8, imm: u32) -> u32 {
    // imm layout in decode val: [20][10:1][11][19:12]
    // Note: current decode stores only 12 valid bits for J-type imm; we only encode from provided bits.
    let bit_20 = ((imm >> 20) & 0x1) as u32;
    let bits_10_1 = ((imm >> 1) & 0x3FF) as u32; // 10 bits
    let bit_11 = ((imm >> 11) & 0x1) as u32;
    let bits_19_12 = ((imm >> 12) & 0xFF) as u32; // 8 bits

    (bit_20 << 31)
        | (bits_19_12 << 12)
        | (bit_11 << 20)
        | (bits_10_1 << 21)
        | ((rd as u32) << 7)
        | opcode
}

#[inline]
fn validate_shamt(shamt: u32, max_bits: u32) -> Result<(), String> {
    if shamt < (1 << max_bits) { Ok(()) } else { Err(format!("shamt {} out of range for {}-bit", shamt, max_bits)) }
}

pub fn encode_u32(inst: &Instruction, _xlen: Xlen) -> Result<u32, String> {
    match inst {
        Instruction::RV32I(i) => encode_rv32i(i),
        Instruction::RV64I(i) => encode_rv64i(i),
        Instruction::RVZicsr(csr) => encode_zicsr(csr),
        Instruction::RV32A(a) => encode_rv32a(a),
        // RVF/RVC/A extensions will be added later
        Instruction::RVC(_) => Err("RVC (compressed) encoding is not yet supported".into()),
        Instruction::RVF(_) => Err("RVF encoding is not yet supported".into()),
        Instruction::RV64A(_) | Instruction::RV128A(_) => Err("RV64A/RV128A encoding is not yet supported".into()),
    }
}

fn encode_rv32i(i: &RV32I) -> Result<u32, String> {
    use RV32I::*;
    Ok(match i {
        Lui(u)   => u_type(OPCODE_LUI, u.rd, u.imm.low_u32()),
        Auipc(u) => u_type(OPCODE_AUIPC, u.rd, u.imm.low_u32()),

        // J-type (note: current Imm width limitation in decode; partial support)
        Jal(j)   => j_type(OPCODE_JAL, j.rd, j.imm.low_u32()),

        // I-type jumps/loads
        Jalr(i)  => i_type(OPCODE_JALR, i.rd, 0, i.rs1, i.imm.low_u32()),

        // Branches (B-type)
        Beq(b) => b_type(OPCODE_BRANCH, FUNCT3_BRANCH_BEQ, b.rs1, b.rs2, b.imm.low_u32()),
        Bne(b) => b_type(OPCODE_BRANCH, FUNCT3_BRANCH_BNE, b.rs1, b.rs2, b.imm.low_u32()),
        Blt(b) => b_type(OPCODE_BRANCH, FUNCT3_BRANCH_BLT, b.rs1, b.rs2, b.imm.low_u32()),
        Bge(b) => b_type(OPCODE_BRANCH, FUNCT3_BRANCH_BGE, b.rs1, b.rs2, b.imm.low_u32()),
        Bltu(b)=> b_type(OPCODE_BRANCH, FUNCT3_BRANCH_BLTU, b.rs1, b.rs2, b.imm.low_u32()),
        Bgeu(b)=> b_type(OPCODE_BRANCH, FUNCT3_BRANCH_BGEU, b.rs1, b.rs2, b.imm.low_u32()),

        // Loads (I-type)
        Lb(i)  => i_type(OPCODE_LOAD, i.rd, FUNCT3_LOAD_LB,  i.rs1, i.imm.low_u32()),
        Lh(i)  => i_type(OPCODE_LOAD, i.rd, FUNCT3_LOAD_LH,  i.rs1, i.imm.low_u32()),
        Lw(i)  => i_type(OPCODE_LOAD, i.rd, FUNCT3_LOAD_LW,  i.rs1, i.imm.low_u32()),
        Lbu(i) => i_type(OPCODE_LOAD, i.rd, FUNCT3_LOAD_LBU, i.rs1, i.imm.low_u32()),
        Lhu(i) => i_type(OPCODE_LOAD, i.rd, FUNCT3_LOAD_LHU, i.rs1, i.imm.low_u32()),

        // Stores (S-type)
        Sb(s) => s_type(OPCODE_STORE, FUNCT3_STORE_SB, s.rs1, s.rs2, s.imm.low_u32()),
        Sh(s) => s_type(OPCODE_STORE, FUNCT3_STORE_SH, s.rs1, s.rs2, s.imm.low_u32()),
        Sw(s) => s_type(OPCODE_STORE, FUNCT3_STORE_SW, s.rs1, s.rs2, s.imm.low_u32()),

        // OP-IMM
        Addi(i)=> i_type(OPCODE_OP_IMM, i.rd, FUNCT3_OP_ADD_SUB, i.rs1, i.imm.low_u32()),
        Slti(i)=> i_type(OPCODE_OP_IMM, i.rd, FUNCT3_OP_SLT, i.rs1, i.imm.low_u32()),
        Sltiu(i)=>i_type(OPCODE_OP_IMM, i.rd, FUNCT3_OP_SLTU, i.rs1, i.imm.low_u32()),
        Xori(i)=> i_type(OPCODE_OP_IMM, i.rd, FUNCT3_OP_XOR, i.rs1, i.imm.low_u32()),
        Ori(i) => i_type(OPCODE_OP_IMM, i.rd, FUNCT3_OP_OR, i.rs1, i.imm.low_u32()),
        Andi(i)=> i_type(OPCODE_OP_IMM, i.rd, FUNCT3_OP_AND, i.rs1, i.imm.low_u32()),

        Slli(i)=> {
            let shamt = i.imm.low_u32() & 0x1f;
            validate_shamt(shamt, 5)?;
            // funct7 = 0 for RV32I slli
            let imm12 = shamt; // high bits are zero
            i_type(OPCODE_OP_IMM, i.rd, FUNCT3_OP_SLL, i.rs1, imm12)
        }
        Srli(i)=> {
            let shamt = i.imm.low_u32() & 0x1f;
            validate_shamt(shamt, 5)?;
            let imm12 = shamt | ((FUNCT7_OP_SRL as u32) << 5);
            i_type(OPCODE_OP_IMM, i.rd, FUNCT3_OP_SRL_SRA, i.rs1, imm12)
        }
        Srai(i)=> {
            let shamt = i.imm.low_u32() & 0x1f;
            validate_shamt(shamt, 5)?;
            let imm12 = shamt | ((FUNCT7_OP_SRA as u32) << 5);
            i_type(OPCODE_OP_IMM, i.rd, FUNCT3_OP_SRL_SRA, i.rs1, imm12)
        }

        // OP (R-type)
        Add(r) => r_type(OPCODE_OP, r.rd, FUNCT3_OP_ADD_SUB, r.rs1, r.rs2, FUNCT7_OP_ADD),
        Sub(r) => r_type(OPCODE_OP, r.rd, FUNCT3_OP_ADD_SUB, r.rs1, r.rs2, FUNCT7_OP_SUB),
        Sll(r) => r_type(OPCODE_OP, r.rd, FUNCT3_OP_SLL, r.rs1, r.rs2, 0),
        Slt(r) => r_type(OPCODE_OP, r.rd, FUNCT3_OP_SLT, r.rs1, r.rs2, 0),
        Sltu(r)=> r_type(OPCODE_OP, r.rd, FUNCT3_OP_SLTU, r.rs1, r.rs2, 0),
        Xor(r) => r_type(OPCODE_OP, r.rd, FUNCT3_OP_XOR, r.rs1, r.rs2, 0),
        Srl(r) => r_type(OPCODE_OP, r.rd, FUNCT3_OP_SRL_SRA, r.rs1, r.rs2, 0),
        Sra(r) => r_type(OPCODE_OP, r.rd, FUNCT3_OP_SRL_SRA, r.rs1, r.rs2, FUNCT7_OP_SRA),
        Or(r)  => r_type(OPCODE_OP, r.rd, FUNCT3_OP_OR, r.rs1, r.rs2, 0),
        And(r) => r_type(OPCODE_OP, r.rd, FUNCT3_OP_AND, r.rs1, r.rs2, 0),

        // M-extension (R-type with funct7 = 0b000_0001)
        Mul(r)   => r_type(OPCODE_OP, r.rd, FUNCT3_OP_ADD_SUB, r.rs1, r.rs2, 0b000_0001),
        Mulh(r)  => r_type(OPCODE_OP, r.rd, FUNCT3_OP_SLL,     r.rs1, r.rs2, 0b000_0001),
        Mulhsu(r)=> r_type(OPCODE_OP, r.rd, FUNCT3_OP_XOR,     r.rs1, r.rs2, 0b000_0001),
        Mulhu(r) => r_type(OPCODE_OP, r.rd, FUNCT3_OP_SLTU,    r.rs1, r.rs2, 0b000_0001),
        Div(r)   => r_type(OPCODE_OP, r.rd, FUNCT3_OP_SRL_SRA, r.rs1, r.rs2, 0b000_0001),
        Divu(r)  => r_type(OPCODE_OP, r.rd, FUNCT3_OP_OR,      r.rs1, r.rs2, 0b000_0001),
        Rem(r)   => r_type(OPCODE_OP, r.rd, FUNCT3_OP_AND,     r.rs1, r.rs2, 0b000_0001),
        Remu(r)  => r_type(OPCODE_OP, r.rd, FUNCT3_OP_AND,     r.rs1, r.rs2, 0b000_0001),

        // System
        Fence(())  => i_type(OPCODE_MISC_MEM, 0, FUNCT3_MISC_MEM_FENCE, 0, 0),
        FenceI(()) => i_type(OPCODE_MISC_MEM, 0, FUNCT3_MISC_MEM_FENCE_I, 0, 0),
        Ecall(())  => i_type(OPCODE_SYSTEM, 0, FUNCT3_SYSTEM_PRIV, 0, FUNCT12_SYSTEM_ECALL),
        Ebreak(()) => i_type(OPCODE_SYSTEM, 0, FUNCT3_SYSTEM_PRIV, 0, FUNCT12_SYSTEM_EBREAK),
    })
}

fn encode_rv64i(i: &RV64I) -> Result<u32, String> {
    use RV64I::*;
    Ok(match i {
        // loads/stores
        Lwu(i) => i_type(OPCODE_LOAD, i.rd, FUNCT3_LOAD_LWU, i.rs1, i.imm.low_u32()),
        Ld(i)  => i_type(OPCODE_LOAD, i.rd, FUNCT3_LOAD_LD,  i.rs1, i.imm.low_u32()),
        Sd(s)  => s_type(OPCODE_STORE, FUNCT3_STORE_SD, s.rs1, s.rs2, s.imm.low_u32()),

        // R-type shifts
        Sll(r) => r_type(OPCODE_OP, r.rd, FUNCT3_OP_SLL, r.rs1, r.rs2, 0),
        Srl(r) => r_type(OPCODE_OP, r.rd, FUNCT3_OP_SRL_SRA, r.rs1, r.rs2, 0),
        Sra(r) => r_type(OPCODE_OP, r.rd, FUNCT3_OP_SRL_SRA, r.rs1, r.rs2, FUNCT7_OP_SRA),

        // I-type shifts (RV64I: 6-bit shamt), top 6 bits in funct7 except LSB holds shamt[5]
        Slli(i)=> {
            let shamt = i.imm.low_u32() & 0x3f;
            validate_shamt(shamt, 6)?;
            let imm_low5 = shamt & 0x1f;
            let funct7 = (0u8 as u8) | ((shamt >> 5) as u8 & 0x1);
            // Build instruction manually to control funct7 bit0 location
            let mut ins = 0u32;
            ins |= (funct7 as u32) << 25;
            ins |= imm_low5 << 20;
            ins |= (i.rs1 as u32) << 15;
            ins |= (FUNCT3_OP_SLL as u32) << 12;
            ins |= (i.rd as u32) << 7;
            ins |= OPCODE_OP_IMM;
            ins
        }
        Srli(i)=> {
            let shamt = i.imm.low_u32() & 0x3f;
            validate_shamt(shamt, 6)?;
            let imm_low5 = shamt & 0x1f;
            let funct7 = (FUNCT7_OP_SRL as u8) | ((shamt >> 5) as u8 & 0x1);
            let mut ins = 0u32;
            ins |= (funct7 as u32) << 25;
            ins |= imm_low5 << 20;
            ins |= (i.rs1 as u32) << 15;
            ins |= (FUNCT3_OP_SRL_SRA as u32) << 12;
            ins |= (i.rd as u32) << 7;
            ins |= OPCODE_OP_IMM;
            ins
        }
        Srai(i)=> {
            let shamt = i.imm.low_u32() & 0x3f;
            validate_shamt(shamt, 6)?;
            let imm_low5 = shamt & 0x1f;
            let funct7 = (FUNCT7_OP_SRA as u8) | ((shamt >> 5) as u8 & 0x1);
            let mut ins = 0u32;
            ins |= (funct7 as u32) << 25;
            ins |= imm_low5 << 20;
            ins |= (i.rs1 as u32) << 15;
            ins |= (FUNCT3_OP_SRL_SRA as u32) << 12;
            ins |= (i.rd as u32) << 7;
            ins |= OPCODE_OP_IMM;
            ins
        }

        // OP-IMM-32 / OP-32 (W variants, 5-bit shamt)
        Addiw(i)=> i_type(OPCODE_OP_IMM32, i.rd, FUNCT3_OP_ADD_SUB, i.rs1, i.imm.low_u32()),
        Slliw(i)=> {
            let shamt = i.imm.low_u32() & 0x1f; validate_shamt(shamt, 5)?;
            let imm12 = shamt; // funct7=0
            i_type(OPCODE_OP_IMM32, i.rd, FUNCT3_OP_SLL, i.rs1, imm12)
        }
        Srliw(i)=> {
            let shamt = i.imm.low_u32() & 0x1f; validate_shamt(shamt, 5)?;
            let imm12 = shamt | ((FUNCT7_OP_SRL as u32) << 5);
            i_type(OPCODE_OP_IMM32, i.rd, FUNCT3_OP_SRL_SRA, i.rs1, imm12)
        }
        Sraiw(i)=> {
            let shamt = i.imm.low_u32() & 0x1f; validate_shamt(shamt, 5)?;
            let imm12 = shamt | ((FUNCT7_OP_SRA as u32) << 5);
            i_type(OPCODE_OP_IMM32, i.rd, FUNCT3_OP_SRL_SRA, i.rs1, imm12)
        }

        Addw(r) => r_type(OPCODE_OP_32, r.rd, FUNCT3_OP_ADD_SUB, r.rs1, r.rs2, FUNCT7_OP_ADD),
        Subw(r) => r_type(OPCODE_OP_32, r.rd, FUNCT3_OP_ADD_SUB, r.rs1, r.rs2, FUNCT7_OP_SUB),
        Sllw(r) => r_type(OPCODE_OP_32, r.rd, FUNCT3_OP_SLL, r.rs1, r.rs2, 0),
        Srlw(r) => r_type(OPCODE_OP_32, r.rd, FUNCT3_OP_SRL_SRA, r.rs1, r.rs2, 0),
        Sraw(r) => r_type(OPCODE_OP_32, r.rd, FUNCT3_OP_SRL_SRA, r.rs1, r.rs2, FUNCT7_OP_SRA),
    })
}

fn encode_zicsr(csr: &RVZicsr) -> Result<u32, String> {
    use RVZicsr::*;
    Ok(match csr {
        Csrrw(c) => i_type(OPCODE_SYSTEM, c.rd, FUNCT3_SYSTEM_CSRRW, c.rs1, c.csr as u32),
        Csrrs(c) => i_type(OPCODE_SYSTEM, c.rd, FUNCT3_SYSTEM_CSRRS, c.rs1, c.csr as u32),
        Csrrc(c) => i_type(OPCODE_SYSTEM, c.rd, FUNCT3_SYSTEM_CSRRC, c.rs1, c.csr as u32),
        // Immediate forms: rs1 field carries uimm[4:0]
        Csrrwi(c)=> i_type(OPCODE_SYSTEM, c.rd, FUNCT3_SYSTEM_CSRRWI, c.uimm.low32() as u8, c.csr as u32),
        Csrrsi(c)=> i_type(OPCODE_SYSTEM, c.rd, FUNCT3_SYSTEM_CSRRSI, c.uimm.low32() as u8, c.csr as u32),
        Csrrci(c)=> i_type(OPCODE_SYSTEM, c.rd, FUNCT3_SYSTEM_CSRRCI, c.uimm.low32() as u8, c.csr as u32),
    })
}

fn encode_rv32a(a: &RV32A) -> Result<u32, String> {
    use RV32A::*;
    // For RV32A, all instructions use OPCODE_A with funct3 = FUNCT3_LOAD_LW (0b010)
    // funct7 for A-extension uses the upper 5 bits (funct5) as the operation code
    Ok(match a {
        Lrw(r) => r_type(OPCODE_A, r.rd, FUNCT3_LOAD_LW, r.rs1, r.rs2, (FUNCT5_A_LR as u8) << 2),
        Scw(r) => r_type(OPCODE_A, r.rd, FUNCT3_LOAD_LW, r.rs1, r.rs2, (FUNCT5_A_SC as u8) << 2),
        Amoswapw(r) => r_type(OPCODE_A, r.rd, FUNCT3_LOAD_LW, r.rs1, r.rs2, (FUNCT5_A_AMOSWAP as u8) << 2),
        Amoaddw(r) => r_type(OPCODE_A, r.rd, FUNCT3_LOAD_LW, r.rs1, r.rs2, (FUNCT5_A_AMOADD as u8) << 2),
        Amoxorw(r) => r_type(OPCODE_A, r.rd, FUNCT3_LOAD_LW, r.rs1, r.rs2, (FUNCT5_A_AMOXOR as u8) << 2),
        Amoandw(r) => r_type(OPCODE_A, r.rd, FUNCT3_LOAD_LW, r.rs1, r.rs2, (FUNCT5_A_AMOAND as u8) << 2),
        Amoorw(r) => r_type(OPCODE_A, r.rd, FUNCT3_LOAD_LW, r.rs1, r.rs2, (FUNCT5_A_AMOOR as u8) << 2),
        Amominw(r) => r_type(OPCODE_A, r.rd, FUNCT3_LOAD_LW, r.rs1, r.rs2, (FUNCT5_A_AMOMIN as u8) << 2),
        Amomaxw(r) => r_type(OPCODE_A, r.rd, FUNCT3_LOAD_LW, r.rs1, r.rs2, (FUNCT5_A_AMOMAX as u8) << 2),
        Amominuw(r) => r_type(OPCODE_A, r.rd, FUNCT3_LOAD_LW, r.rs1, r.rs2, (FUNCT5_A_AMOMINU as u8) << 2),
        Amomaxuw(r) => r_type(OPCODE_A, r.rd, FUNCT3_LOAD_LW, r.rs1, r.rs2, (FUNCT5_A_AMOMAXU as u8) << 2),
    })
}
