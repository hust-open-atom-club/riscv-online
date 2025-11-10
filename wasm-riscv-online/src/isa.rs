//! Shared RISC-V ISA constants used by decoders/encoders.
#![allow(dead_code)]

// =========================
// 32-bit instruction opcodes
// =========================
pub const OPCODE_LOAD: u32 = 0b000_0011;
pub const OPCODE_LOAD_FP: u32 = 0b000_0111;
pub const OPCODE_MISC_MEM: u32 = 0b000_1111;
pub const OPCODE_OP_IMM: u32 = 0b001_0011;
pub const OPCODE_AUIPC: u32 = 0b001_0111;
pub const OPCODE_OP_IMM32: u32 = 0b001_1011;
pub const OPCODE_STORE: u32 = 0b010_0011;
pub const OPCODE_STORE_FP: u32 = 0b010_0111;
pub const OPCODE_OP: u32 = 0b011_0011;
pub const OPCODE_LUI: u32 = 0b011_0111;
pub const OPCODE_OP_32: u32 = 0b011_1011;
pub const OPCODE_FMADD: u32 = 0b100_0011;
pub const OPCODE_FMSUB: u32 = 0b100_0111;
pub const OPCODE_FNMSUB: u32 = 0b100_1011;
pub const OPCODE_FNMADD: u32 = 0b100_1111;
pub const OPCODE_FP: u32 = 0b101_0011;
pub const OPCODE_BRANCH: u32 = 0b110_0011;
pub const OPCODE_JALR: u32 = 0b110_0111;
pub const OPCODE_JAL: u32 = 0b110_1111;
pub const OPCODE_SYSTEM: u32 = 0b111_0011;
pub const OPCODE_A: u32 = 0b010_1111;

// =========================
// funct3 fields (loads/stores/branches/ALU)
// =========================
// LOAD
pub const FUNCT3_LOAD_LB: u8 = 0b000;
pub const FUNCT3_LOAD_LH: u8 = 0b001;
pub const FUNCT3_LOAD_LW: u8 = 0b010;
pub const FUNCT3_LOAD_LD: u8 = 0b011;
pub const FUNCT3_LOAD_LBU: u8 = 0b100;
pub const FUNCT3_LOAD_LHU: u8 = 0b101;
pub const FUNCT3_LOAD_LWU: u8 = 0b110;

// STORE
pub const FUNCT3_STORE_SB: u8 = 0b000;
pub const FUNCT3_STORE_SH: u8 = 0b001;
pub const FUNCT3_STORE_SW: u8 = 0b010;
pub const FUNCT3_STORE_SD: u8 = 0b011;

// BRANCH
pub const FUNCT3_BRANCH_BEQ: u8 = 0b000;
pub const FUNCT3_BRANCH_BNE: u8 = 0b001;
pub const FUNCT3_BRANCH_BLT: u8 = 0b100;
pub const FUNCT3_BRANCH_BGE: u8 = 0b101;
pub const FUNCT3_BRANCH_BLTU: u8 = 0b110;
pub const FUNCT3_BRANCH_BGEU: u8 = 0b111;

// OP/OP-IMM (ALU)
pub const FUNCT3_OP_ADD_SUB: u8 = 0b000;
pub const FUNCT3_OP_SLL: u8 = 0b001;
pub const FUNCT3_OP_SLT: u8 = 0b010;
pub const FUNCT3_OP_SLTU: u8 = 0b011;
pub const FUNCT3_OP_XOR: u8 = 0b100;
pub const FUNCT3_OP_SRL_SRA: u8 = 0b101;
pub const FUNCT3_OP_OR: u8 = 0b110;
pub const FUNCT3_OP_AND: u8 = 0b111;

// =========================
// funct7 / funct12
// =========================
pub const FUNCT7_OP_SRL: u8 = 0b000_0000;
pub const FUNCT7_OP_SRA: u8 = 0b010_0000;

pub const FUNCT7_OP_ADD: u8 = 0b000_0000;
pub const FUNCT7_OP_SUB: u8 = 0b010_0000;

// SYSTEM
pub const FUNCT3_SYSTEM_PRIV: u8 = 0b000;
pub const FUNCT3_SYSTEM_CSRRW: u8 = 0b001;
pub const FUNCT3_SYSTEM_CSRRS: u8 = 0b010;
pub const FUNCT3_SYSTEM_CSRRC: u8 = 0b011;
pub const FUNCT3_SYSTEM_CSRRWI: u8 = 0b101;
pub const FUNCT3_SYSTEM_CSRRSI: u8 = 0b110;
pub const FUNCT3_SYSTEM_CSRRCI: u8 = 0b111;

pub const FUNCT12_SYSTEM_ECALL: u32 = 0b000;
pub const FUNCT12_SYSTEM_EBREAK: u32 = 0b001;

// MISC-MEM
pub const FUNCT3_MISC_MEM_FENCE: u8 = 0b000;
pub const FUNCT3_MISC_MEM_FENCE_I: u8 = 0b001;

// width
pub const FUNCT3_WIDTH_W: u8 = 0b010;

// =========================
// Floating-point encodings (RVF/RVD)
// =========================
pub const FUNCT2_FMT_S: u8 = 0b00; // single-precision
pub const FUNCT2_FMT_D: u8 = 0b01; // double-precision

pub const FUNCT_RS3_FP_ADD: u8 = 0b00000;
pub const FUNCT_RS3_FP_SUB: u8 = 0b00001;
pub const FUNCT_RS3_FP_MUL: u8 = 0b00010;
pub const FUNCT_RS3_FP_DIV: u8 = 0b00011;
pub const FUNCT_RS3_FP_SGNJ: u8 = 0b00100;
pub const FUNCT_RS3_FP_MIN_MAX: u8 = 0b00101;
pub const FUNCT_RS3_FP_SQRT: u8 = 0b01011;
pub const FUNCT_RS3_FP_CMP: u8 = 0b10100;
pub const FUNCT_RS3_FP_FCVTX: u8 = 0b11000; // fcvt.{w|l}[u].s, fcvt.int.fmt
pub const FUNCT_RS3_FP_XCVTF: u8 = 0b11010; // fcvt.s.{w|l}[u], fcvt.fmt.int
pub const FUNCT_RS3_FP_FMVX_CLASS: u8 = 0b11100; // fmv.x.w / fclass.s
pub const FUNCT_RS3_FP_XMVF: u8 = 0b11110; // fmv.w.x

pub const FUNCT3_FP_MIN: u8 = 0b000;
pub const FUNCT3_FP_MAX: u8 = 0b001;

pub const FUNCT3_FP_SGNJ: u8 = 0b000;
pub const FUNCT3_FP_SGNJN: u8 = 0b001;
pub const FUNCT3_FP_SGNJX: u8 = 0b010;

pub const FUNCT3_FP_EQ: u8 = 0b010;
pub const FUNCT3_FP_LT: u8 = 0b001;
pub const FUNCT3_FP_LE: u8 = 0b000;

pub const FUNCT_RS2_CVT_W: u8 = 0b00000;
pub const FUNCT_RS2_CVT_WU: u8 = 0b00001;
pub const FUNCT_RS2_CVT_L: u8 = 0b00010;
pub const FUNCT_RS2_CVT_LU: u8 = 0b00011;

// =========================
// Atomic (A-extension)
// =========================
pub const FUNCT5_A_AMOADD: u8 = 0b00000;
pub const FUNCT5_A_AMOSWAP: u8 = 0b00001;
pub const FUNCT5_A_LR: u8 = 0b00010;
pub const FUNCT5_A_SC: u8 = 0b00011;
pub const FUNCT5_A_AMOXOR: u8 = 0b00100;
pub const FUNCT5_A_AMOOR: u8 = 0b01000;
pub const FUNCT5_A_AMOAND: u8 = 0b01100;
pub const FUNCT5_A_AMOMIN: u8 = 0b10000;
pub const FUNCT5_A_AMOMAX: u8 = 0b10100;
pub const FUNCT5_A_AMOMINU: u8 = 0b11000;
pub const FUNCT5_A_AMOMAXU: u8 = 0b11100;

// A-extension width for RV128A (.q)
pub const FUNCT3_A_WIDTH_Q: u8 = 0b100;

// =========================
// Bit Manipulation (B-extension)
// =========================
pub const FUNCT3_BSET: u8 = 0b000;
pub const FUNCT3_BEXT: u8 = 0b001;
pub const FUNCT3_BINV: u8 = 0b010;
pub const FUNCT3_GREVI: u8 = 0b010;
pub const FUNCT3_GORC: u8 = 0b010;
pub const FUNCT3_ANDN: u8 = 0b000;
pub const FUNCT3_ORN: u8 = 0b001;
pub const FUNCT3_XNOR: u8 = 0b010;
pub const FUNCT3_ROR: u8 = 0b101;
pub const FUNCT3_BSETW: u8 = 0b000;
pub const FUNCT3_BEXTW: u8 = 0b001;
pub const FUNCT3_BINVW: u8 = 0b010;
pub const FUNCT3_GREVIW: u8 = 0b010;
pub const FUNCT3_GORCW: u8 = 0b010;
pub const FUNCT3_RORW: u8 = 0b101;

// B-extension funct7 values
pub const FUNCT7_B_BSET: u8 = 0b0010000;
pub const FUNCT7_B_BEXT: u8 = 0b0010000;
pub const FUNCT7_B_BINV: u8 = 0b0010000;
pub const FUNCT7_B_BSETI: u8 = 0b0010000;
pub const FUNCT7_B_BEXTI: u8 = 0b0010000;
pub const FUNCT7_B_BINVI: u8 = 0b0010000;
pub const FUNCT7_B_GREVI: u8 = 0b0010000;
pub const FUNCT7_B_GREVIW: u8 = 0b0010000;
pub const FUNCT7_B_GORC: u8 = 0b0010000;
pub const FUNCT7_B_GORCW: u8 = 0b0010000;
pub const FUNCT7_B_GORHI: u8 = 0b0010100;
pub const FUNCT7_B_GORHIW: u8 = 0b0010100;
pub const FUNCT7_B_GXORI: u8 = 0b0010100;
pub const FUNCT7_B_GXORIW: u8 = 0b0010100;
pub const FUNCT7_B_ANDN: u8 = 0b0110000;
pub const FUNCT7_B_ORN: u8 = 0b0110000;
pub const FUNCT7_B_XNOR: u8 = 0b0110000;
pub const FUNCT7_B_ROR: u8 = 0b0110000;
pub const FUNCT7_B_RORW: u8 = 0b0110000;
pub const FUNCT7_B_RORI: u8 = 0b0110000;
pub const FUNCT7_B_RORIW: u8 = 0b0110000;
pub const FUNCT7_B_CPOP: u8 = 0b0110000;
pub const FUNCT7_B_CPOPW: u8 = 0b0110000;
pub const FUNCT7_B_CTZ: u8 = 0b0110000;
pub const FUNCT7_B_CTZW: u8 = 0b0110000;
pub const FUNCT7_B_CLZ: u8 = 0b0110000;
pub const FUNCT7_B_CLZW: u8 = 0b0110000;
pub const FUNCT7_B_FFS: u8 = 0b0110000;
pub const FUNCT7_B_FFSW: u8 = 0b0110000;
pub const FUNCT7_B_PACK: u8 = 0b0110000;
pub const FUNCT7_B_PACKW: u8 = 0b0110000;

// B-extension funct6 values for CR-type instructions
pub const FUNCT6_B_CLZ: u8 = 0b101001;
pub const FUNCT6_B_CTZ: u8 = 0b101010;
pub const FUNCT6_B_CPOP: u8 = 0b101011;
pub const FUNCT6_B_PACK: u8 = 0b101100;
pub const FUNCT6_B_PACKW: u8 = 0b101101;
pub const FUNCT6_B_FFS: u8 = 0b101110;

// =========================
// Compressed (RVC) opcodes (2-bit)
// =========================
pub const OPCODE_C0: u16 = 0b00;
pub const OPCODE_C1: u16 = 0b01;
pub const OPCODE_C2: u16 = 0b10;

// RVD-specific funct5/funct7 values
pub const FUNCT_RS3_FP_CVTD: u8 = 0b11000; // fcvt.{w|l}[u].d
pub const FUNCT_RS3_FP_XCVTFD: u8 = 0b11010; // fcvt.d.{w|l}[u]
pub const FUNCT_RS3_FP_FMVXD: u8 = 0b11100; // fmv.x.d / fclass.d
pub const FUNCT_RS3_FP_XMVFD: u8 = 0b11110; // fmv.d.x
pub const FUNCT_RS3_FP_FCVTSD: u8 = 0b11000; // for fcvt.s.d and fcvt.d.s

