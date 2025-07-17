use crate::{asm::Instruction,asm::*, riscv::imm::{Imm, Uimm, Xlen}};

const OPCODE_LOAD: u32 = 0b000_0011;
const OPCODE_LOAD_FP: u32 = 0b000_0111;
const OPCODE_MISC_MEM: u32 = 0b000_1111;
const OPCODE_OP_IMM: u32 = 0b001_0011;
const OPCODE_AUIPC: u32 = 0b001_0111;
const OPCODE_OP_IMM32: u32 = 0b001_1011;
const OPCODE_STORE: u32 = 0b010_0011;
const OPCODE_STORE_FP: u32 = 0b010_0111;
const OPCODE_OP: u32 = 0b011_0011;
const OPCODE_LUI: u32 = 0b011_0111;
const OPCODE_OP_32: u32 = 0b011_1011;
const OPCODE_FMADD: u32 = 0b100_0011;
const OPCODE_FMSUB: u32 = 0b100_0111;
const OPCODE_FNMSUB: u32 = 0b100_1011;
const OPCODE_FNMADD: u32 = 0b100_1111;
const OPCODE_FP: u32 = 0b101_0011;
const OPCODE_BRANCH: u32 = 0b110_0011;
const OPCODE_JALR: u32 = 0b110_0111;
const OPCODE_JAL: u32 = 0b110_1111;
const OPCODE_SYSTEM: u32 = 0b111_0011;

const FUNCT3_LOAD_LB: u8 = 0b000;
const FUNCT3_LOAD_LH: u8 = 0b001;
const FUNCT3_LOAD_LW: u8 = 0b010;
const FUNCT3_LOAD_LD: u8 = 0b011;
const FUNCT3_LOAD_LBU: u8 = 0b100;
const FUNCT3_LOAD_LHU: u8 = 0b101;
const FUNCT3_LOAD_LWU: u8 = 0b110;

const FUNCT3_STORE_SB: u8 = 0b000;
const FUNCT3_STORE_SH: u8 = 0b001;
const FUNCT3_STORE_SW: u8 = 0b010;
const FUNCT3_STORE_SD: u8 = 0b011;

const FUNCT3_BRANCH_BEQ: u8 = 0b000;
const FUNCT3_BRANCH_BNE: u8 = 0b001;
const FUNCT3_BRANCH_BLT: u8 = 0b100;
const FUNCT3_BRANCH_BGE: u8 = 0b101;
const FUNCT3_BRANCH_BLTU: u8 = 0b110;
const FUNCT3_BRANCH_BGEU: u8 = 0b111;

const FUNCT3_OP_ADD_SUB: u8 = 0b000;
const FUNCT3_OP_SLL: u8 = 0b001;
const FUNCT3_OP_SLT: u8 = 0b010;
const FUNCT3_OP_SLTU: u8 = 0b011;
const FUNCT3_OP_XOR: u8 = 0b100;
const FUNCT3_OP_SRL_SRA: u8 = 0b101;
const FUNCT3_OP_OR: u8 = 0b110;
const FUNCT3_OP_AND: u8 = 0b111;

const FUNCT7_OP_SRL: u8 = 0b000_0000;
const FUNCT7_OP_SRA: u8 = 0b010_0000;

const FUNCT7_OP_ADD: u8 = 0b000_0000;
const FUNCT7_OP_SUB: u8 = 0b010_0000;

const FUNCT3_SYSTEM_PRIV: u8 = 0b000;
const FUNCT3_SYSTEM_CSRRW: u8 = 0b001;
const FUNCT3_SYSTEM_CSRRS: u8 = 0b010;
const FUNCT3_SYSTEM_CSRRC: u8 = 0b011;
const FUNCT3_SYSTEM_CSRRWI: u8 = 0b101;
const FUNCT3_SYSTEM_CSRRSI: u8 = 0b110;
const FUNCT3_SYSTEM_CSRRCI: u8 = 0b111;

const FUNCT12_SYSTEM_ECALL: u32 = 0b000;
const FUNCT12_SYSTEM_EBREAK: u32 = 0b001;

const FUNCT3_MISC_MEM_FENCE: u8 = 0b000;

const FUNCT3_WIDTH_W: u8 = 0b010;

const FUNCT2_FMT_S: u8 = 0b00;

const FUNCT_RS3_FP_ADD: u8 = 0b00000;
const FUNCT_RS3_FP_SUB: u8 = 0b00001;
const FUNCT_RS3_FP_MUL: u8 = 0b00010;
const FUNCT_RS3_FP_DIV: u8 = 0b00011;
const FUNCT_RS3_FP_SGNJ: u8 = 0b00100;
const FUNCT_RS3_FP_MIN_MAX: u8 = 0b00101;
const FUNCT_RS3_FP_SQRT: u8 = 0b01011;
const FUNCT_RS3_FP_CMP: u8 = 0b10100;
const FUNCT_RS3_FP_FCVTX: u8 = 0b11000; // fcvt.{w|l}[u].s, fcvt.int.fmt
const FUNCT_RS3_FP_XCVTF: u8 = 0b11010; // fcvt.s.{w|l}[u], fcvt.fmt.int
const FUNCT_RS3_FP_FMVX_CLASS: u8 = 0b11100; // fmv.x.w
const FUNCT_RS3_FP_XMVF: u8 = 0b11110; // fmv.w.x

const FUNCT3_FP_MIN: u8 = 0b000;
const FUNCT3_FP_MAX: u8 = 0b001;

const FUNCT3_FP_SGNJ: u8 = 0b000;
const FUNCT3_FP_SGNJN: u8 = 0b001;
const FUNCT3_FP_SGNJX: u8 = 0b010;

const FUNCT3_FP_EQ: u8 = 0b010;
const FUNCT3_FP_LT: u8 = 0b001;
const FUNCT3_FP_LE: u8 = 0b000;

const FUNCT_RS2_CVT_W: u8 = 0b00000;
const FUNCT_RS2_CVT_WU: u8 = 0b00001;
const FUNCT_RS2_CVT_L: u8 = 0b00010;
const FUNCT_RS2_CVT_LU: u8 = 0b00011;

pub fn resolve_u32(ins: u32, xlen: Xlen) -> core::result::Result<Instruction, ()> {
    use crate::asm::{RVZicsr::*, RV32I::*, RV64I::*, RVF::*};
    let opcode = ins & 0b111_1111;
    let rd = ((ins >> 7) & 0b1_1111) as u8;
    let rs1 = ((ins >> 15) & 0b1_1111) as u8;
    let rs2 = ((ins >> 20) & 0b1_1111) as u8;
    let funct3 = ((ins >> 12) & 0b111) as u8;
    let funct7 = ((ins >> 25) & 0b111_1111) as u8;
    let funct12 = (ins >> 20) & 0b1111_1111_1111;
    let rs3 = ((ins >> 27) & 0b1_1111) as u8;
    let funct2 = ((ins >> 25) & 0b11) as u8;
    let imm_i = {
        let val = (ins >> 20) & 0b1111_1111_1111;
        Imm::new(val, 12)
    };
    let imm_s = {
        let val = ((ins >> 7) & 0b11111) | (((ins >> 25) & 0b1111111) << 5);
        Imm::new(val, 12)
    };
    let imm_b = {
        let val = (((ins >> 7) & 0b1) << 11)
            | (((ins >> 8) & 0b1111) << 1)
            | (((ins >> 25) & 0b111111) << 5)
            | (((ins >> 31) & 0b1) << 12);
        Imm::new(val, 12)
    };
    let imm_u = Imm::new(ins & 0xFFFFF000, 32);
    let imm_j = {
        let val = (((ins & 0b1000_0000_0000_0000_0000_0000_0000_0000) >> 31) << 20)
            | (((ins & 0b0111_1111_1110_0000_0000_0000_0000_0000) >> 21) << 1)
            | (((ins & 0b0000_0000_0001_0000_0000_0000_0000_0000) >> 20) << 11)
            | (((ins & 0b0000_0000_0000_1111_1111_0000_0000_0000) >> 12) << 12);
        Imm::new(val, 12)
    };
    let uimm_csr = Uimm::new((ins >> 15) & 0b11111, 5);
    let csr = ((ins >> 20) & 0xFFF) as u16;
    let u_type = UType { rd, imm: imm_u };
    let j_type = JType { rd, imm: imm_j };
    let b_type = BType {
        rs1,
        rs2,
        funct3,
        imm: imm_b,
    };
    let i_type = IType {
        rd,
        rs1,
        funct3,
        imm: imm_i,
    };
    let s_type = SType {
        rs1,
        rs2,
        funct3,
        imm: imm_s,
    };
    let r_type = RType {
        rd,
        rs1,
        rs2,
        funct3,
        funct7,
    };
    let csr_r_type = CsrRType {
        rd,
        rs1,
        funct3,
        csr,
    };
    let csr_i_type = CsrIType {
        rd,
        uimm: uimm_csr,
        funct3,
        csr,
    };
    let r4_type = R4Type {
        rd,
        rs1,
        rs2,
        rs3,
        funct3,
        funct2,
    };
    let ans = match opcode {
        OPCODE_LUI => Lui(u_type).into(),
        OPCODE_AUIPC => Auipc(u_type).into(),
        OPCODE_JAL => Jal(j_type).into(),
        OPCODE_JALR => Jalr(i_type).into(),
        OPCODE_BRANCH => match funct3 {
            FUNCT3_BRANCH_BEQ => Beq(b_type).into(),
            FUNCT3_BRANCH_BNE => Bne(b_type).into(),
            FUNCT3_BRANCH_BLT => Blt(b_type).into(),
            FUNCT3_BRANCH_BGE => Bge(b_type).into(),
            FUNCT3_BRANCH_BLTU => Bltu(b_type).into(),
            FUNCT3_BRANCH_BGEU => Bgeu(b_type).into(),
            _ => Err(())?,
        },
        OPCODE_LOAD => match funct3 {
            FUNCT3_LOAD_LB => Lb(i_type).into(),
            FUNCT3_LOAD_LH => Lh(i_type).into(),
            FUNCT3_LOAD_LW => Lw(i_type).into(),
            FUNCT3_LOAD_LD if xlen != Xlen::X32 => Ld(i_type).into(),
            FUNCT3_LOAD_LBU => Lbu(i_type).into(),
            FUNCT3_LOAD_LHU => Lhu(i_type).into(),
            FUNCT3_LOAD_LWU if xlen != Xlen::X32 => Lwu(i_type).into(),
            _ => Err(())?,
        },
        OPCODE_STORE => match funct3 {
            FUNCT3_STORE_SB => Sb(s_type).into(),
            FUNCT3_STORE_SH => Sh(s_type).into(),
            FUNCT3_STORE_SW => Sw(s_type).into(),
            FUNCT3_STORE_SD if xlen != Xlen::X32 => Sd(s_type).into(),
            _ => Err(())?,
        },
        OPCODE_MISC_MEM => match funct3 {
            FUNCT3_MISC_MEM_FENCE => Fence(i_type).into(),
            _ => Err(())?,
        },
        OPCODE_SYSTEM => match funct3 {
            FUNCT3_SYSTEM_PRIV => match funct12 {
                FUNCT12_SYSTEM_ECALL if funct3 == FUNCT3_SYSTEM_PRIV && rs1 == 0 && rd == 0 => {
                    Ecall(i_type).into()
                }
                FUNCT12_SYSTEM_EBREAK if funct3 == FUNCT3_SYSTEM_PRIV && rs1 == 0 && rd == 0 => {
                    Ebreak(i_type).into()
                }
                _ => Err(())?,
            },
            FUNCT3_SYSTEM_CSRRW => Csrrw(csr_r_type).into(),
            FUNCT3_SYSTEM_CSRRS => Csrrs(csr_r_type).into(),
            FUNCT3_SYSTEM_CSRRC => Csrrc(csr_r_type).into(),
            FUNCT3_SYSTEM_CSRRWI => Csrrwi(csr_i_type).into(),
            FUNCT3_SYSTEM_CSRRSI => Csrrsi(csr_i_type).into(),
            FUNCT3_SYSTEM_CSRRCI => Csrrci(csr_i_type).into(),
            _ => Err(())?,
        },
        OPCODE_OP_IMM => match funct3 {
            FUNCT3_OP_ADD_SUB => Addi(i_type).into(),
            FUNCT3_OP_SLT => Slti(i_type).into(),
            FUNCT3_OP_SLTU => Sltiu(i_type).into(),
            FUNCT3_OP_XOR => Xori(i_type).into(),
            FUNCT3_OP_OR => Ori(i_type).into(),
            FUNCT3_OP_AND => Andi(i_type).into(),
            FUNCT3_OP_SLL if funct7 == 0 && xlen == Xlen::X32 => RV32I::Slli(i_type).into(),
            FUNCT3_OP_SLL if funct7 & 0b1111110 == 0 && xlen == Xlen::X64 => {
                RV64I::Slli(i_type).into()
            }
            FUNCT3_OP_SRL_SRA => match funct7 {
                FUNCT7_OP_SRL if xlen == Xlen::X32 => RV32I::Srli(i_type).into(),
                FUNCT7_OP_SRA if xlen == Xlen::X32 => RV32I::Srai(i_type).into(),
                x if x & 0b1111110 == FUNCT7_OP_SRL && xlen == Xlen::X64 => {
                    RV64I::Srli(i_type).into()
                }
                x if x & 0b1111110 == FUNCT7_OP_SRA && xlen == Xlen::X64 => {
                    RV64I::Srai(i_type).into()
                }
                _ => Err(())?,
            },
            _ => Err(())?,
        },
        OPCODE_OP => match funct3 {
            FUNCT3_OP_ADD_SUB => match funct7 {
                FUNCT7_OP_ADD => Add(r_type).into(),
                FUNCT7_OP_SUB => Sub(r_type).into(),
                0b000_0001 => Mul(r_type).into(),
                _ => Err(())?,
            },
            FUNCT3_OP_SLL => match funct7 {
                0 => RV32I::Sll(r_type).into(),
                0b000_0001 => Mulh(r_type).into(),
                _ => Err(())?,
            },
            FUNCT3_OP_SLT if funct7 == 0 => Slt(r_type).into(),
            FUNCT3_OP_SLTU if funct7 == 0 => Sltu(r_type).into(),
            FUNCT3_OP_XOR => match funct7 {
                0 => Xor(r_type).into(),
                0b000_0001 => Mulhsu(r_type).into(),
                _ => Err(())?,
            },
            FUNCT3_OP_SRL_SRA => match funct7 {
                0 => RV32I::Srl(r_type).into(),
                0b010_0000 => RV32I::Sra(r_type).into(),
                0b000_0001 => Div(r_type).into(),
                _ => Err(())?,
            },
            FUNCT3_OP_OR => match funct7 {
                0 => Or(r_type).into(),
                0b000_0001 => Divu(r_type).into(),
                _ => Err(())?,
            },
            FUNCT3_OP_AND => match funct7 {
                0 => And(r_type).into(),
                0b000_0001 if xlen == Xlen::X32 => Rem(r_type).into(),
                0b000_0001 if xlen == Xlen::X64 => Remu(r_type).into(),
                _ => Err(())?,
            },
            _ => Err(())?,
        },
        OPCODE_OP_IMM32 if xlen == Xlen::X64 => match funct3 {
            FUNCT3_OP_ADD_SUB => Addiw(i_type).into(),
            FUNCT3_OP_SLL if funct7 == 0 => Slliw(i_type).into(),
            FUNCT3_OP_SRL_SRA => match funct7 {
                FUNCT7_OP_SRL => Srliw(i_type).into(),
                FUNCT7_OP_SRA => Sraiw(i_type).into(),
                _ => Err(())?,
            },
            _ => Err(())?,
        },
        OPCODE_OP_32 if xlen == Xlen::X64 => match funct3 {
            FUNCT3_OP_ADD_SUB => match funct7 {
                FUNCT7_OP_ADD => Addw(r_type).into(),
                FUNCT7_OP_SUB => Subw(r_type).into(),
                _ => Err(())?,
            },
            FUNCT3_OP_SLL if funct7 == 0 => Sllw(r_type).into(),
            FUNCT3_OP_SRL_SRA => match funct7 {
                FUNCT7_OP_SRL => Srlw(r_type).into(),
                FUNCT7_OP_SRA => Sraw(r_type).into(),
                _ => Err(())?,
            },
            _ => Err(())?,
        },
        OPCODE_LOAD_FP => match funct3 {
            FUNCT3_WIDTH_W => Flw(i_type).into(),
            _ => Err(())?,
        },
        OPCODE_STORE_FP => match funct3 {
            FUNCT3_WIDTH_W => Fsw(s_type).into(),
            _ => Err(())?,
        },
        OPCODE_FMADD => match funct2 {
            FUNCT2_FMT_S => Fmadds(r4_type).into(),
            _ => Err(())?,
        },
        OPCODE_FMSUB => match funct2 {
            FUNCT2_FMT_S => Fmsubs(r4_type).into(),
            _ => Err(())?,
        },
        OPCODE_FNMSUB => match funct2 {
            FUNCT2_FMT_S => Fnmsubs(r4_type).into(),
            _ => Err(())?,
        },
        OPCODE_FNMADD => match funct2 {
            FUNCT2_FMT_S => Fnmadds(r4_type).into(),
            _ => Err(())?,
        },
        OPCODE_FP => match rs3 {
            FUNCT_RS3_FP_ADD => match funct2 {
                FUNCT2_FMT_S => Fadds(r_type).into(),
                _ => Err(())?,
            },
            FUNCT_RS3_FP_SUB => match funct2 {
                FUNCT2_FMT_S => Fsubs(r_type).into(),
                _ => Err(())?,
            },
            FUNCT_RS3_FP_MUL => match funct2 {
                FUNCT2_FMT_S => Fmuls(r_type).into(),
                _ => Err(())?,
            },
            FUNCT_RS3_FP_DIV => match funct2 {
                FUNCT2_FMT_S => Fdivs(r_type).into(),
                _ => Err(())?,
            },
            FUNCT_RS3_FP_SQRT if rs2 == 0 => match funct2 {
                FUNCT2_FMT_S => Fsqrts(r_type).into(),
                _ => Err(())?,
            },
            FUNCT_RS3_FP_MIN_MAX => match funct3 {
                FUNCT3_FP_MIN => match funct2 {
                    FUNCT2_FMT_S => Fmins(r_type).into(),
                    _ => Err(())?,
                },
                FUNCT3_FP_MAX => match funct2 {
                    FUNCT2_FMT_S => Fmaxs(r_type).into(),
                    _ => Err(())?,
                },
                _ => Err(())?,
            },
            FUNCT_RS3_FP_SGNJ => match funct3 {
                FUNCT3_FP_SGNJ => match funct2 {
                    FUNCT2_FMT_S => Fsgnjs(r_type).into(),
                    _ => Err(())?,
                },
                FUNCT3_FP_SGNJN => match funct2 {
                    FUNCT2_FMT_S => Fsgnjns(r_type).into(),
                    _ => Err(())?,
                },
                FUNCT3_FP_SGNJX => match funct2 {
                    FUNCT2_FMT_S => Fsgnjxs(r_type).into(),
                    _ => Err(())?,
                },
                _ => Err(())?,
            },
            FUNCT_RS3_FP_CMP => match funct3 {
                FUNCT3_FP_EQ => match funct2 {
                    FUNCT2_FMT_S => Feqs(r_type).into(),
                    _ => Err(())?,
                },
                FUNCT3_FP_LT => match funct2 {
                    FUNCT2_FMT_S => Flts(r_type).into(),
                    _ => Err(())?,
                },
                FUNCT3_FP_LE => match funct2 {
                    FUNCT2_FMT_S => Fles(r_type).into(),
                    _ => Err(())?,
                },
                _ => Err(())?,
            },
            // fcvt.{w|l}[u].s, fcvt.int.fmt
            FUNCT_RS3_FP_FCVTX => match rs2 {
                FUNCT_RS2_CVT_W => match funct2 {
                    FUNCT2_FMT_S => Fcvtws(r_type).into(),
                    _ => Err(())?,
                },
                FUNCT_RS2_CVT_WU => match funct2 {
                    FUNCT2_FMT_S => Fcvtwus(r_type).into(),
                    _ => Err(())?,
                },
                FUNCT_RS2_CVT_L if xlen != Xlen::X32 => match funct2 {
                    FUNCT2_FMT_S => Fcvtls(r_type).into(),
                    _ => Err(())?,
                },
                FUNCT_RS2_CVT_LU if xlen != Xlen::X32 => match funct2 {
                    FUNCT2_FMT_S => Fcvtlus(r_type).into(),
                    _ => Err(())?,
                },
                _ => Err(())?,
            },
            // fcvt.s.{w|l}[u], fcvt.fmt.int
            FUNCT_RS3_FP_XCVTF => match rs2 {
                FUNCT_RS2_CVT_W => match funct2 {
                    FUNCT2_FMT_S => Fcvtsw(r_type).into(),
                    _ => Err(())?,
                },
                FUNCT_RS2_CVT_WU => match funct2 {
                    FUNCT2_FMT_S => Fcvtswu(r_type).into(),
                    _ => Err(())?,
                },
                FUNCT_RS2_CVT_L if xlen != Xlen::X32 => match funct2 {
                    FUNCT2_FMT_S => Fcvtsl(r_type).into(),
                    _ => Err(())?,
                },
                FUNCT_RS2_CVT_LU if xlen != Xlen::X32 => match funct2 {
                    FUNCT2_FMT_S => Fcvtslu(r_type).into(),
                    _ => Err(())?,
                },
                _ => Err(())?,
            },
            // fmv.x.w
            FUNCT_RS3_FP_FMVX_CLASS if rs2 == 0 && funct3 == 0 => match funct2 {
                FUNCT2_FMT_S => Fmvxw(r_type).into(),
                _ => Err(())?,
            },
            FUNCT_RS3_FP_FMVX_CLASS if rs2 == 0 && funct3 == 1 => match funct2 {
                FUNCT2_FMT_S => Fclasss(r_type).into(),
                _ => Err(())?,
            },
            // fmv.w.x
            FUNCT_RS3_FP_XMVF if rs2 == 0 && funct3 == 0 => match funct2 {
                FUNCT2_FMT_S => Fmvwx(r_type).into(),
                _ => Err(())?,
            },
            _ => Err(())?,
        }, // opcode_fp
        _ => Err(())?,
    };
    Ok(ans)
}
