use crate::{asm::{Instruction, RV32I, RV64I, RVF, RVZicsr, RV32A, RV64A, RV128A, RV64D, RVB, CsrRType, CsrIType, R4Type, IType, SType, RType, UType, JType, BType}, riscv::imm::{Imm, Uimm, Xlen}};
use crate::isa::*;

pub fn resolve_u32(ins: u32, xlen: Xlen) -> core::result::Result<Instruction, ()> {
    // 不使用通配符导入以避免命名冲突
    let opcode = ins & 0b111_1111;
    let rd = ((ins >> 7) & 0b1_1111) as u8;
    let rs1 = ((ins >> 15) & 0b1_1111) as u8;
    let rs2 = ((ins >> 20) & 0b1_1111) as u8;
    let funct3 = ((ins >> 12) & 0b111) as u8;
    let funct5 = ((ins >> 27) & 0b11111) as u8;
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
        OPCODE_LUI => RV32I::Lui(u_type).into(),
        OPCODE_AUIPC => RV32I::Auipc(u_type).into(),
        OPCODE_JAL => RV32I::Jal(j_type).into(),
        OPCODE_JALR => RV32I::Jalr(i_type).into(),
        OPCODE_BRANCH => match funct3 {
            FUNCT3_BRANCH_BEQ => RV32I::Beq(b_type).into(),
            FUNCT3_BRANCH_BNE => RV32I::Bne(b_type).into(),
            FUNCT3_BRANCH_BLT => RV32I::Blt(b_type).into(),
            FUNCT3_BRANCH_BGE => RV32I::Bge(b_type).into(),
            FUNCT3_BRANCH_BLTU => RV32I::Bltu(b_type).into(),
            FUNCT3_BRANCH_BGEU => RV32I::Bgeu(b_type).into(),
            _ => Err(())?,
        },
        // B-extension instructions
        OPCODE_OP if funct7 == FUNCT7_B_ANDN => match funct3 {
            FUNCT3_ANDN => RVB::Andn(r_type).into(),
            FUNCT3_ORN => RVB::Orn(r_type).into(),
            FUNCT3_XNOR => RVB::Xnor(r_type).into(),
            FUNCT3_ROR => RVB::Ror(r_type).into(),
            _ => Err(())?,
        },
        OPCODE_OP_32 if xlen != Xlen::X32 && funct7 == FUNCT7_B_RORW => match funct3 {
            FUNCT3_RORW => RVB::Rorw(r_type).into(),
            _ => Err(())?,
        },
        OPCODE_OP_IMM if funct7 == FUNCT7_B_RORI => match funct3 {
            FUNCT3_ROR => RVB::Rori(i_type).into(),
            _ => Err(())?,
        },
        OPCODE_OP_IMM32 if xlen != Xlen::X32 && funct7 == FUNCT7_B_RORIW => match funct3 {
            FUNCT3_RORW => RVB::Roriw(i_type).into(),
            _ => Err(())?,
        },
        OPCODE_OP_32 if xlen != Xlen::X32 && funct7 == 0 => match funct3 {
            FUNCT3_OP_SLL => RV64I::Sllw(r_type).into(),
            FUNCT3_OP_SRL_SRA => match funct7 {
                FUNCT7_OP_SRL => RV64I::Srlw(r_type).into(),
                FUNCT7_OP_SRA => RV64I::Sraw(r_type).into(),
                _ => Err(())?,
            },
            _ => Err(())?,
        },
        // 位操作指令 - B-extension
        OPCODE_OP if funct7 == FUNCT7_B_BSET => match funct3 {
            FUNCT3_BSET => RVB::Bset(r_type).into(),
            FUNCT3_BEXT => RVB::Bext(r_type).into(),
            FUNCT3_BINV => RVB::Binv(r_type).into(),
            _ => Err(())?,
        },
        OPCODE_OP_32 if xlen != Xlen::X32 && funct7 == FUNCT7_B_BSET => match funct3 {
            FUNCT3_BSETW => RVB::BsetW(r_type).into(),
            FUNCT3_BEXTW => RVB::BextW(r_type).into(),
            FUNCT3_BINVW => RVB::BinvW(r_type).into(),
            _ => Err(())?,
        },
        OPCODE_OP_IMM if funct7 == FUNCT7_B_BSETI => match funct3 {
            FUNCT3_BSET => RVB::Bseti(i_type).into(),
            FUNCT3_BEXT => RVB::Bexti(i_type).into(),
            FUNCT3_BINV => RVB::Binvi(i_type).into(),
            _ => Err(())?,
        },
        OPCODE_OP_IMM32 if xlen != Xlen::X32 && funct7 == FUNCT7_B_BSETI => match funct3 {
            FUNCT3_BSETW => RVB::BsetiW(i_type).into(),
            FUNCT3_BEXTW => RVB::BextiW(i_type).into(),
            FUNCT3_BINVW => RVB::BinviW(i_type).into(),
            _ => Err(())?,
        },
        // 位计数和包指令
        OPCODE_OP if funct7 == FUNCT7_B_CPOP => match funct3 {
            0 => match rs2 & 0b11111 {
                0 => RVB::Cpop(r_type).into(),
                1 => RVB::Ctz(r_type).into(),
                2 => RVB::Clz(r_type).into(),
                3 => RVB::Pack(r_type).into(),
                4 => RVB::FFS(r_type).into(),
                _ => Err(())?,
            },
            _ => Err(())?,
        },
        OPCODE_OP_32 if xlen != Xlen::X32 && funct7 == FUNCT7_B_CPOPW => match funct3 {
            0 => match rs2 & 0b11111 {
                0 => RVB::Cpopw(r_type).into(),
                1 => RVB::CtzW(r_type).into(),
                2 => RVB::ClzW(r_type).into(),
                3 => RVB::PackW(r_type).into(),
                4 => RVB::FFSW(r_type).into(),
                _ => Err(())?,
            },
            _ => Err(())?,
        },
        OPCODE_LOAD => match funct3 {
            FUNCT3_LOAD_LB => RV32I::Lb(i_type).into(),
            FUNCT3_LOAD_LH => RV32I::Lh(i_type).into(),
            FUNCT3_LOAD_LW => RV32I::Lw(i_type).into(),
            FUNCT3_LOAD_LD if xlen != Xlen::X32 => RV64I::Ld(i_type).into(),
            FUNCT3_LOAD_LBU => RV32I::Lbu(i_type).into(),
            FUNCT3_LOAD_LHU => RV32I::Lhu(i_type).into(),
            FUNCT3_LOAD_LWU if xlen != Xlen::X32 => RV64I::Lwu(i_type).into(),
            _ => Err(())?,
        },
        OPCODE_STORE => match funct3 {
            FUNCT3_STORE_SB => RV32I::Sb(s_type).into(),
            FUNCT3_STORE_SH => RV32I::Sh(s_type).into(),
            FUNCT3_STORE_SW => RV32I::Sw(s_type).into(),
            FUNCT3_STORE_SD if xlen != Xlen::X32 => RV64I::Sd(s_type).into(),
            _ => Err(())?,
        },
        OPCODE_MISC_MEM => match funct3 {
            FUNCT3_MISC_MEM_FENCE => RV32I::Fence(()).into(),
            FUNCT3_MISC_MEM_FENCE_I => RV32I::FenceI(()).into(),
            _ => Err(())?,
        },
        OPCODE_SYSTEM => match funct3 {
            FUNCT3_SYSTEM_PRIV => match funct12 {
                FUNCT12_SYSTEM_ECALL if funct3 == FUNCT3_SYSTEM_PRIV && rs1 == 0 && rd == 0 => {
                    RV32I::Ecall(()).into()
                }
                FUNCT12_SYSTEM_EBREAK if funct3 == FUNCT3_SYSTEM_PRIV && rs1 == 0 && rd == 0 => {
                    RV32I::Ebreak(()).into()
                }
                _ => Err(())?,
            },
            FUNCT3_SYSTEM_CSRRW => RVZicsr::Csrrw(csr_r_type).into(),
            FUNCT3_SYSTEM_CSRRS => RVZicsr::Csrrs(csr_r_type).into(),
            FUNCT3_SYSTEM_CSRRC => RVZicsr::Csrrc(csr_r_type).into(),
            FUNCT3_SYSTEM_CSRRWI => RVZicsr::Csrrwi(csr_i_type).into(),
            FUNCT3_SYSTEM_CSRRSI => RVZicsr::Csrrsi(csr_i_type).into(),
            FUNCT3_SYSTEM_CSRRCI => RVZicsr::Csrrci(csr_i_type).into(),
            _ => Err(())?,
        },
        OPCODE_OP_IMM => match funct3 {
            FUNCT3_OP_ADD_SUB => RV32I::Addi(i_type).into(),
            FUNCT3_OP_SLT => RV32I::Slti(i_type).into(),
            FUNCT3_OP_SLTU => RV32I::Sltiu(i_type).into(),
            FUNCT3_OP_XOR => RV32I::Xori(i_type).into(),
            FUNCT3_OP_OR => RV32I::Ori(i_type).into(),
            FUNCT3_OP_AND => RV32I::Andi(i_type).into(),
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
                FUNCT7_OP_ADD => RV32I::Add(r_type).into(),
                FUNCT7_OP_SUB => RV32I::Sub(r_type).into(),
                0b000_0001 => RV32I::Mul(r_type).into(),
                _ => Err(())?,
            },
            FUNCT3_OP_SLL => match funct7 {
                0 => RV32I::Sll(r_type).into(),
                0b000_0001 => RV32I::Mulh(r_type).into(),
                _ => Err(())?,
            },
            FUNCT3_OP_SLT if funct7 == 0 => RV32I::Slt(r_type).into(),
            FUNCT3_OP_SLTU if funct7 == 0 => RV32I::Sltu(r_type).into(),
            FUNCT3_OP_XOR => match funct7 {
                0 => RV32I::Xor(r_type).into(),
                0b000_0001 => RV32I::Mulhsu(r_type).into(),
                _ => Err(())?,
            },
            FUNCT3_OP_SRL_SRA => match funct7 {
                0 => RV32I::Srl(r_type).into(),
                0b010_0000 => RV32I::Sra(r_type).into(),
                0b000_0001 => RV32I::Div(r_type).into(),
                _ => Err(())?,
            },
            FUNCT3_OP_OR => match funct7 {
                0 => RV32I::Or(r_type).into(),
                0b000_0001 => RV32I::Divu(r_type).into(),
                _ => Err(())?,
            },
            FUNCT3_OP_AND => match funct7 {
                0 => RV32I::And(r_type).into(),
                0b000_0001 if xlen == Xlen::X32 => RV32I::Rem(r_type).into(),
                0b000_0001 if xlen == Xlen::X64 => RV32I::Remu(r_type).into(),
                _ => Err(())?,
            },
            _ => Err(())?,
        },
        OPCODE_OP_IMM32 if xlen == Xlen::X64 => match funct3 {
            FUNCT3_OP_ADD_SUB => RV64I::Addiw(i_type).into(),
            FUNCT3_OP_SLL if funct7 == 0 => RV64I::Slliw(i_type).into(),
            FUNCT3_OP_SRL_SRA => match funct7 {
                FUNCT7_OP_SRL => RV64I::Srliw(i_type).into(),
                FUNCT7_OP_SRA => RV64I::Sraiw(i_type).into(),
                _ => Err(())?,
            },
            _ => Err(())?,
        },
        OPCODE_OP_32 if xlen == Xlen::X64 => match funct3 {
            FUNCT3_OP_ADD_SUB => match funct7 {
                FUNCT7_OP_ADD => RV64I::Addw(r_type).into(),
                FUNCT7_OP_SUB => RV64I::Subw(r_type).into(),
                _ => Err(())?,
            },
            FUNCT3_OP_SLL if funct7 == 0 => RV64I::Sllw(r_type).into(),
            FUNCT3_OP_SRL_SRA => match funct7 {
                FUNCT7_OP_SRL => RV64I::Srlw(r_type).into(),
                FUNCT7_OP_SRA => RV64I::Sraw(r_type).into(),
                _ => Err(())?,
            },
            _ => Err(())?,
        },
        OPCODE_LOAD_FP => match funct3 {
            FUNCT3_WIDTH_W => RVF::Flw(i_type).into(),
            FUNCT3_LOAD_LD if xlen != Xlen::X32 => RV64D::Fld(i_type).into(),
            _ => Err(())?,
        },
        OPCODE_STORE_FP => match funct3 {
            FUNCT3_WIDTH_W => RVF::Fsw(s_type).into(),
            FUNCT3_STORE_SD if xlen != Xlen::X32 => RV64D::Fsd(s_type).into(),
            _ => Err(())?,
        },
        OPCODE_FMADD => match funct2 {
            FUNCT2_FMT_S => RVF::Fmadds(r4_type).into(),
            FUNCT2_FMT_D => RV64D::FmaddD(r4_type).into(),
            _ => Err(())?,
        },
        OPCODE_FMSUB => match funct2 {
            FUNCT2_FMT_S => RVF::Fmsubs(r4_type).into(),
            FUNCT2_FMT_D => RV64D::FmsubD(r4_type).into(),
            _ => Err(())?,
        },
        OPCODE_FNMSUB => match funct2 {
            FUNCT2_FMT_S => RVF::Fnmsubs(r4_type).into(),
            FUNCT2_FMT_D => RV64D::FnmsubD(r4_type).into(),
            _ => Err(())?,
        },
        OPCODE_FNMADD => match funct2 {
            FUNCT2_FMT_S => RVF::Fnmadds(r4_type).into(),
            FUNCT2_FMT_D => RV64D::FnmaddD(r4_type).into(),
            _ => Err(())?,
        },
        OPCODE_FP => match rs3 {
            FUNCT_RS3_FP_ADD => match funct2 {
                FUNCT2_FMT_S => RVF::Fadds(r_type).into(),
                _ => Err(())?,
            },
            FUNCT_RS3_FP_SUB => match funct2 {
                FUNCT2_FMT_S => RVF::Fsubs(r_type).into(),
                _ => Err(())?,
            },
            FUNCT_RS3_FP_MUL => match funct2 {
                FUNCT2_FMT_S => RVF::Fmuls(r_type).into(),
                _ => Err(())?,
            },
            FUNCT_RS3_FP_DIV => match funct2 {
                FUNCT2_FMT_S => RVF::Fdivs(r_type).into(),
                _ => Err(())?,
            },
            FUNCT_RS3_FP_SQRT if rs2 == 0 => match funct2 {
                FUNCT2_FMT_S => RVF::Fsqrts(r_type).into(),
                _ => Err(())?,
            },
            FUNCT_RS3_FP_MIN_MAX => match funct3 {
                FUNCT3_FP_MIN => match funct2 {
                    FUNCT2_FMT_S => RVF::Fmins(r_type).into(),
                    _ => Err(())?,
                },
                FUNCT3_FP_MAX => match funct2 {
                    FUNCT2_FMT_S => RVF::Fmaxs(r_type).into(),
                    _ => Err(())?,
                },
                _ => Err(())?,
            },
            FUNCT_RS3_FP_SGNJ => match funct3 {
                FUNCT3_FP_SGNJ => {
                    match funct2 {
                        FUNCT2_FMT_S => RVF::Fsgnjs(r_type).into(),
                        _ => Err(())?
                    }
                },
                FUNCT3_FP_SGNJN => {
                    match funct2 {
                        FUNCT2_FMT_S => RVF::Fsgnjns(r_type).into(),
                        _ => Err(())?
                    }
                },
                FUNCT3_FP_SGNJX => {
                    match funct2 {
                        FUNCT2_FMT_S => RVF::Fsgnjxs(r_type).into(),
                        _ => Err(())?
                    }
                },
                _ => Err(())?
            },
            FUNCT_RS3_FP_CMP => match funct3 {
                FUNCT3_FP_EQ => match funct2 {
                    FUNCT2_FMT_S => RVF::Feqs(r_type).into(),
                    _ => Err(())?,
                },
                FUNCT3_FP_LT => match funct2 {
                    FUNCT2_FMT_S => RVF::Flts(r_type).into(),
                    _ => Err(())?,
                },
                FUNCT3_FP_LE => match funct2 {
                    FUNCT2_FMT_S => RVF::Fles(r_type).into(),
                    FUNCT2_FMT_D => RV64D::FleD(r_type).into(),
                    _ => Err(())?,
                },
                _ => Err(())?,
            },
            // fcvt.{w|l}[u].s, fcvt.int.fmt
            FUNCT_RS3_FP_FCVTX => match rs2 {
                FUNCT_RS2_CVT_W => match funct2 {
                    FUNCT2_FMT_S => RVF::Fcvtws(r_type).into(),
                    _ => Err(())?,
                },
                FUNCT_RS2_CVT_WU => match funct2 {
                    FUNCT2_FMT_S => RVF::Fcvtwus(r_type).into(),
                    _ => Err(())?,
                },
                FUNCT_RS2_CVT_L if xlen != Xlen::X32 => match funct2 {
                    FUNCT2_FMT_S => RVF::Fcvtls(r_type).into(),
                    _ => Err(())?,
                },
                FUNCT_RS2_CVT_LU if xlen != Xlen::X32 => match funct2 {
                    FUNCT2_FMT_S => RVF::Fcvtslu(r_type).into(),
                    FUNCT2_FMT_D => RV64D::FcvtDLU(r_type).into(),
                    _ => Err(())?,
                },
                _ => Err(())?,
            },
            // fcvt.s.{w|l}[u], fcvt.fmt.int
            FUNCT_RS3_FP_XCVTF => match rs2 {
                FUNCT_RS2_CVT_W => match funct2 {
                    FUNCT2_FMT_S => RVF::Fcvtsw(r_type).into(),
                    _ => Err(())?,
                },
                FUNCT_RS2_CVT_WU => match funct2 {
                    FUNCT2_FMT_S => RVF::Fcvtswu(r_type).into(),
                    _ => Err(())?,
                },
                FUNCT_RS2_CVT_L if xlen != Xlen::X32 => match funct2 {
                    FUNCT2_FMT_S => RVF::Fcvtsl(r_type).into(),
                    _ => Err(())?,
                },
                FUNCT_RS2_CVT_LU if xlen != Xlen::X32 => match funct2 {
                    FUNCT2_FMT_S => RVF::Fcvtslu(r_type).into(),
                    FUNCT2_FMT_D => RV64D::FcvtLUD(r_type).into(),
                    _ => Err(())?,
                },
                // 单精度和双精度之间的转换
                FUNCT_RS3_FP_FCVTSD if rs2 == 0 => match funct2 {
                    FUNCT2_FMT_D => RV64D::FcvtSD(r_type).into(), // fcvt.s.d
                    FUNCT2_FMT_S => RV64D::FcvtDS(r_type).into(), // fcvt.d.s
                    _ => Err(())?,
                },
                _ => Err(())?,
            },
            // fmv.x.w/d
            FUNCT_RS3_FP_FMVX_CLASS if rs2 == 0 && funct3 == 0 => match funct2 {
                FUNCT2_FMT_S => RVF::Fmvxw(r_type).into(),
                FUNCT2_FMT_D => RV64D::FmvXD(r_type).into(),
                _ => Err(())?,
            },
            FUNCT_RS3_FP_FMVX_CLASS if rs2 == 0 && funct3 == 1 => match funct2 {
                FUNCT2_FMT_S => RVF::Fclasss(r_type).into(),
                FUNCT2_FMT_D => RV64D::FclassD(r_type).into(),
                _ => Err(())?,
            },
            // fmv.w/d.x
            FUNCT_RS3_FP_XMVF if rs2 == 0 && funct3 == 0 => match funct2 {
                FUNCT2_FMT_S => RVF::Fmvwx(r_type).into(),
                FUNCT2_FMT_D => RV64D::FmvDX(r_type).into(),
                _ => Err(())?,
            },
            _ => Err(())?,
        }, // opcode_fp

        // atomic instructions
        OPCODE_A => match funct3 {
            FUNCT3_LOAD_LW => match funct5 {
                FUNCT5_A_LR => RV32A::Lrw(r_type).into(),
                FUNCT5_A_SC => RV32A::Scw(r_type).into(),
                FUNCT5_A_AMOSWAP => RV32A::Amoswapw(r_type).into(),
                FUNCT5_A_AMOADD => RV32A::Amoaddw(r_type).into(),
                FUNCT5_A_AMOXOR => RV32A::Amoxorw(r_type).into(),
                FUNCT5_A_AMOAND => RV32A::Amoandw(r_type).into(),
                FUNCT5_A_AMOOR => RV32A::Amoorw(r_type).into(),
                FUNCT5_A_AMOMIN => RV32A::Amominw(r_type).into(),
                FUNCT5_A_AMOMAX => RV32A::Amomaxw(r_type).into(),
                FUNCT5_A_AMOMINU => RV32A::Amominuw(r_type).into(),
                FUNCT5_A_AMOMAXU => RV32A::Amomaxuw(r_type).into(),
                _ => Err(())?,
            },
            FUNCT3_LOAD_LD => match funct5 {
                FUNCT5_A_LR => RV64A::Lrd(r_type).into(),
                FUNCT5_A_SC => RV64A::Scd(r_type).into(),
                FUNCT5_A_AMOSWAP => RV64A::Amoswapd(r_type).into(),
                FUNCT5_A_AMOADD => RV64A::Amoaddd(r_type).into(),
                FUNCT5_A_AMOXOR => RV64A::Amoxord(r_type).into(),
                FUNCT5_A_AMOAND => RV64A::Amoandd(r_type).into(),
                FUNCT5_A_AMOOR => RV64A::Amoord(r_type).into(),
                FUNCT5_A_AMOMIN => RV64A::Amomind(r_type).into(),
                FUNCT5_A_AMOMAX => RV64A::Amomaxd(r_type).into(),
                FUNCT5_A_AMOMINU => RV64A::Amominud(r_type).into(),
                FUNCT5_A_AMOMAXU => RV64A::Amomaxud(r_type).into(),
                _ => Err(())?,
            },
            // RV128A (.q) width
            x if x == FUNCT3_A_WIDTH_Q && xlen == Xlen::X128 => match funct5 {
                FUNCT5_A_LR => RV128A::Lrq(r_type).into(),
                FUNCT5_A_SC => RV128A::Scq(r_type).into(),
                FUNCT5_A_AMOSWAP => RV128A::Amoswapq(r_type).into(),
                FUNCT5_A_AMOADD => RV128A::Amoaddq(r_type).into(),
                FUNCT5_A_AMOXOR => RV128A::Amoxorq(r_type).into(),
                FUNCT5_A_AMOAND => RV128A::Amoandq(r_type).into(),
                FUNCT5_A_AMOOR => RV128A::Amoorq(r_type).into(),
                FUNCT5_A_AMOMIN => RV128A::Amominq(r_type).into(),
                FUNCT5_A_AMOMAX => RV128A::Amomaxq(r_type).into(),
                FUNCT5_A_AMOMINU => RV128A::Amominuq(r_type).into(),
                FUNCT5_A_AMOMAXU => RV128A::Amomaxuq(r_type).into(),
                _ => Err(())?,
            },
            _ => Err(())?,
        },
        _ => Err(())?,
    };
    Ok(ans)
}
