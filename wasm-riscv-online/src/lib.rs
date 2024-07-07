mod imm;
mod size;
mod utils;

use crate::imm::Imm;
use crate::imm::Uimm;
use crate::imm::Xlen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn disassemble(input: &str) -> String {
    match input_to_u32(input) {
        Ok(value) => {
            if is_16_bit_instruction(value) {
                // Assuming value is truncated to 16-bit for resolve_u16
                if value > 0xFFFF {
                    return format!("Error: invalid 16-bit instruction");
                }
                match resolve_u16((value & 0xFFFF) as u16, Xlen::X32) {
                    Ok(instruction) => instruction.disassembly(),
                    Err(_) => format!("Error: unsupported 16-bit instruction"),
                }
            } else {
                match resolve_u32(value, Xlen::X32) {
                    Ok(instruction) => instruction.disassembly(),
                    Err(_) => format!("Error: unsupported 32-bit instruction"),
                }
            }
        }
        Err(e) => format!("Error: invalid input: {}", e),
    }
}

fn is_16_bit_instruction(value: u32) -> bool {
    // Example logic to determine if the instruction is 16-bit
    // This will vary depending on the actual instruction set specification
    value & 0b11 != 0b11 // Check if the last two bits are not both 1 (indicating a 32-bit instruction)
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, RISCV-ONLINE!");
}

fn input_to_u32(hex_str: &str) -> Result<u32, std::num::ParseIntError> {
    // 检查字符串是否以 "0x" 或 "0X" 开头，并将其剥离
    let trimmed_str = if hex_str.starts_with("0x") || hex_str.starts_with("0X") {
        &hex_str[2..]
    } else {
        hex_str
    };

    // 解析剥离后的字符串为 u32，注意这里的基数是 16
    u32::from_str_radix(trimmed_str, 16)
}

const OPCODE_C0: u16 = 0b00;
const OPCODE_C1: u16 = 0b01;
const OPCODE_C2: u16 = 0b10;

fn resolve_u16(ins: u16, xlen: Xlen) -> core::result::Result<Instruction, ()> {
    use {self::RVC::*, Instruction::*};
    let opcode = ins & 0b11;
    let funct3 = ((ins >> 13) & 0b111) as u8; // keep 0b111 to be explict (actually do not need to & 0b111)
    let funct2 = ((ins >> 5) & 0b11) as u8;
    let funct6 = ((ins >> 10) & 0b111111) as u8;
    let funct4 = ((ins >> 12) & 0b1111) as u8;
    let ins12 = (ins & (1 << 12)) != 0;
    let nzuimm549623 = (((ins >> 11) & 0b11) << 4)
        | (((ins >> 7) & 0b1111) << 6)
        | (((ins >> 6) & 0b1) << 2)
        | (((ins >> 5) & 0b1) << 3);
    let uimm5376 = (((ins >> 10) & 0b111) << 3) | (((ins >> 5) & 0b11) << 6);
    let uimm54876 =
        (((ins >> 11) & 0b11) << 4) | (((ins >> 10) & 0b1) << 8) | (((ins >> 5) & 0b11) << 6);
    let uimm5326 =
        (((ins >> 11) & 0b111) << 3) | (((ins >> 5) & 0b1) << 6) | (((ins >> 6) & 0b1) << 2);
    let nzuimm540 = ((ins >> 2) & 0b11111) | (((ins >> 12) & 0b1) << 5);
    let nzimm540 = nzuimm540;
    let imm540 = nzuimm540;
    let imm114981067315 = (((ins >> 3) & 0b11) << 1)
        | (((ins >> 11) & 0b1) << 3)
        | (((ins >> 2) & 0b1) << 4)
        | (((ins >> 7) & 0b1) << 5)
        | (((ins >> 6) & 0b1) << 6)
        | (((ins >> 9) & 0b11) << 8)
        | (((ins >> 8) & 0b1) << 9)
        | (((ins >> 11) & 0b1) << 10);
    let nzimm946875 = (((ins >> 12) & 0b1) << 9)
        | (((ins >> 6) & 0b1) << 4)
        | (((ins >> 5) & 0b1) << 6)
        | (((ins >> 3) & 0b11) << 7)
        | (((ins >> 2) & 0b1) << 5);
    let nzuimm171612 = {
        let ins = ins as u32;
        (((ins >> 12) & 0b1) << 17) | (((ins >> 2) & 0b11111) << 12)
    };
    let imm84376215 = (((ins >> 12) & 0b1) << 8)
        | (((ins >> 10) & 0b11) << 3)
        | (((ins >> 5) & 0b11) << 6)
        | (((ins >> 3) & 0b11) << 1)
        | (((ins >> 2) & 0b11) << 5);
    let uimm54386 =
        (((ins >> 12) & 0b1) << 5) | (((ins >> 5) & 0b11) << 3) | (((ins >> 2) & 0b111) << 6);
    let uimm5_4_96 =
        (((ins >> 12) & 0b1) << 5) | (((ins >> 6) & 0b1) << 4) | (((ins >> 2) & 0b1111) << 6);
    let uimm54276 =
        (((ins >> 12) & 0b1) << 5) | (((ins >> 4) & 0b111) << 2) | (((ins >> 2) & 0b111) << 6);
    let uimm5386 = (((ins >> 10) & 0b111) << 3) | (((ins >> 7) & 0b111) << 6);
    let uimm54_96 = (((ins >> 11) & 0b11) << 4) | (((ins >> 7) & 0b1111) << 6);
    let uimm5276 = (((ins >> 9) & 0b1111) << 3) | (((ins >> 7) & 0b11) << 6);
    let r24_c = ((ins >> 2) & 0b111) as u8;
    let r79_c = ((ins >> 7) & 0b111) as u8;
    let rdrs1 = ((ins >> 7) & 0b11111) as u8;
    let rs2 = ((ins >> 2) & 0b11111) as u8;
    let ans = match (opcode, funct3) {
        (OPCODE_C0, 0b000) if nzuimm549623 != 0 => RVC(Caddi4spn(CIWType {
            rd: c_reg(r24_c),
            funct3,
            uimm: Uimm::new(nzuimm549623 as u32, 10),
        }))
        .into(),
        (OPCODE_C0, 0b001) if xlen == Xlen::X32 || xlen == Xlen::X64 => RVC(Cfld(CLType {
            rd: c_reg(r24_c),
            rs1: c_reg(r79_c),
            funct3,
            imm: Imm::new(uimm5376 as u32, 8),
        }))
        .into(),
        (OPCODE_C0, 0b001) if xlen == Xlen::X128 => RVC(Clq(CLType {
            rd: c_reg(r24_c),
            rs1: c_reg(r79_c),
            funct3,
            imm: Imm::new(uimm54876 as u32, 9),
        }))
        .into(),
        (OPCODE_C0, 0b010) => RVC(Clw(CLType {
            rd: c_reg(r24_c),
            rs1: c_reg(r79_c),
            funct3,
            imm: Imm::new(uimm5326 as u32, 7),
        }))
        .into(),
        (OPCODE_C0, 0b011) if xlen == Xlen::X32 => RVC(Cflw(CLType {
            rd: c_reg(r24_c),
            rs1: c_reg(r79_c),
            funct3,
            imm: Imm::new(uimm5326 as u32, 7),
        }))
        .into(),
        (OPCODE_C0, 0b011) if xlen == Xlen::X64 || xlen == Xlen::X128 => RVC(Cld(CLType {
            rd: c_reg(r24_c),
            rs1: c_reg(r79_c),
            funct3,
            imm: Imm::new(uimm5376 as u32, 8),
        }))
        .into(),
        (OPCODE_C0, 0b101) if xlen == Xlen::X32 || xlen == Xlen::X64 => RVC(Cfsd(CSType {
            rs1: c_reg(r79_c),
            rs2: c_reg(r24_c),
            funct3,
            imm: Imm::new(uimm5376 as u32, 8),
        }))
        .into(),
        (OPCODE_C0, 0b101) if xlen == Xlen::X128 => RVC(Csq(CSType {
            rs1: c_reg(r79_c),
            rs2: c_reg(r24_c),
            funct3,
            imm: Imm::new(uimm54876 as u32, 9),
        }))
        .into(),
        (OPCODE_C0, 0b110) => RVC(Csw(CSType {
            rs1: c_reg(r79_c),
            rs2: c_reg(r24_c),
            funct3,
            imm: Imm::new(uimm5326 as u32, 7),
        }))
        .into(),
        (OPCODE_C0, 0b111) if xlen == Xlen::X32 => RVC(Cfsw(CSType {
            rs1: c_reg(r79_c),
            rs2: c_reg(r24_c),
            funct3,
            imm: Imm::new(uimm5326 as u32, 7),
        }))
        .into(),
        (OPCODE_C0, 0b111) if xlen == Xlen::X64 || xlen == Xlen::X128 => RVC(Csd(CSType {
            rs1: c_reg(r79_c),
            rs2: c_reg(r24_c),
            funct3,
            imm: Imm::new(uimm5376 as u32, 8),
        }))
        .into(),
        (OPCODE_C1, 0b000) if rdrs1 == 0 => RVC(Cnop(CIType {
            rdrs1,
            funct3,
            imm: Imm::new(nzimm540 as u32, 6),
        }))
        .into(),
        (OPCODE_C1, 0b000) if rdrs1 != 0 => RVC(Caddi(CIType {
            rdrs1,
            funct3,
            imm: Imm::new(nzimm540 as u32, 6),
        }))
        .into(),
        (OPCODE_C1, 0b001) if xlen == Xlen::X32 => RVC(Cjal(CJType {
            funct3,
            target: Imm::new(imm114981067315 as u32, 12),
        }))
        .into(),
        (OPCODE_C1, 0b001) if xlen == Xlen::X64 || xlen == Xlen::X128 => RVC(Caddiw(CIType {
            rdrs1,
            funct3,
            imm: Imm::new(imm540 as u32, 6),
        }))
        .into(),
        (OPCODE_C1, 0b010) if rdrs1 != 0 => RVC(Cli(CIType {
            rdrs1,
            funct3,
            imm: Imm::new(imm540 as u32, 6),
        }))
        .into(),
        (OPCODE_C1, 0b011) if rdrs1 == 2 => RVC(Caddi16sp(CIType {
            rdrs1,
            funct3,
            imm: Imm::new(nzimm946875 as u32, 10),
        }))
        .into(),
        (OPCODE_C1, 0b011) if rdrs1 != 2 && rdrs1 != 0 && nzuimm171612 != 0 => RVC(Clui(CIType {
            rdrs1,
            funct3,
            imm: Imm::new(nzuimm171612, 18),
        }))
        .into(),
        (OPCODE_C1, 0b100) => match (funct6 & 0b11, ins12, funct2) {
            (0b00, _, _) if !(xlen == Xlen::X32 && ins12) && nzuimm540 != 0 => RVC(Csrli(CIType {
                rdrs1: c_reg(r79_c),
                funct3,
                imm: Imm::new(nzuimm540 as u32, 6),
            }))
            .into(),
            (0b00, _, _) if xlen == Xlen::X128 && nzuimm540 == 0 => RVC(Csrli64(CIType {
                rdrs1: c_reg(r79_c),
                funct3,
                imm: Imm::new(nzuimm540 as u32, 6),
            }))
            .into(),
            (0b01, _, _) if !(xlen == Xlen::X32 && ins12) && nzuimm540 != 0 => RVC(Csrai(CIType {
                rdrs1: c_reg(r79_c),
                funct3,
                imm: Imm::new(nzuimm540 as u32, 6),
            }))
            .into(),
            (0b01, _, _) if xlen == Xlen::X128 && nzuimm540 == 0 => RVC(Csrai64(CIType {
                rdrs1: c_reg(r79_c),
                funct3,
                imm: Imm::new(nzuimm540 as u32, 6),
            }))
            .into(),
            (0b10, _, _) => RVC(Candi(CIType {
                rdrs1: c_reg(r79_c),
                funct3,
                imm: Imm::new(imm540 as u32, 6),
            }))
            .into(),
            (0b11, false, 0b00) => RVC(Csub(CAType {
                rdrs1: c_reg(r79_c),
                rs2: c_reg(r24_c),
                funct2,
                funct6,
            }))
            .into(),
            (0b11, false, 0b01) => RVC(Cxor(CAType {
                rdrs1: c_reg(r79_c),
                rs2: c_reg(r24_c),
                funct2,
                funct6,
            }))
            .into(),
            (0b11, false, 0b10) => RVC(Cor(CAType {
                rdrs1: c_reg(r79_c),
                rs2: c_reg(r24_c),
                funct2,
                funct6,
            }))
            .into(),
            (0b11, false, 0b11) => RVC(Cand(CAType {
                rdrs1: c_reg(r79_c),
                rs2: c_reg(r24_c),
                funct2,
                funct6,
            }))
            .into(),
            (0b11, true, 0b00) if xlen == Xlen::X64 || xlen == Xlen::X128 => RVC(Csubw(CAType {
                rdrs1: c_reg(r79_c),
                rs2: c_reg(r24_c),
                funct2,
                funct6,
            }))
            .into(),
            (0b11, true, 0b01) if xlen == Xlen::X64 || xlen == Xlen::X128 => RVC(Caddw(CAType {
                rdrs1: c_reg(r79_c),
                rs2: c_reg(r24_c),
                funct2,
                funct6,
            }))
            .into(),
            _ => Err(())?,
        },
        (OPCODE_C1, 0b101) => RVC(Cj(CJType {
            funct3,
            target: Imm::new(imm114981067315 as u32, 12),
        }))
        .into(),
        (OPCODE_C1, 0b110) => RVC(Cbeqz(CBType {
            rs1: c_reg(r79_c),
            funct3,
            off: Imm::new(imm84376215 as u32, 9),
        }))
        .into(),
        (OPCODE_C1, 0b111) => RVC(Cbnez(CBType {
            rs1: c_reg(r79_c),
            funct3,
            off: Imm::new(imm84376215 as u32, 9),
        }))
        .into(),
        (OPCODE_C2, 0b000) if rdrs1 != 0 && !(xlen == Xlen::X32 && ins12) && nzuimm540 != 0 => {
            RVC(Cslli(CIType {
                rdrs1,
                funct3,
                imm: Imm::new(nzuimm540 as u32, 6),
            }))
            .into()
        }
        (OPCODE_C2, 0b000) if rdrs1 != 0 && xlen == Xlen::X128 && nzuimm540 == 0 => {
            RVC(Cslli64(CIType {
                rdrs1,
                funct3,
                imm: Imm::new(nzuimm540 as u32, 6),
            }))
            .into()
        }
        (OPCODE_C2, 0b001) if xlen == Xlen::X32 || xlen == Xlen::X64 => RVC(Cfldsp(CIType {
            rdrs1,
            funct3,
            imm: Imm::new(uimm54386 as u32, 9),
        }))
        .into(),
        (OPCODE_C2, 0b001) if xlen == Xlen::X128 && rdrs1 != 0 => RVC(Clqsp(CIType {
            rdrs1,
            funct3,
            imm: Imm::new(uimm5_4_96 as u32, 10),
        }))
        .into(),
        (OPCODE_C2, 0b010) if rdrs1 != 0 => RVC(Clwsp(CIType {
            rdrs1,
            funct3,
            imm: Imm::new(uimm54276 as u32, 8),
        }))
        .into(),
        (OPCODE_C2, 0b011) if xlen == Xlen::X32 => RVC(Cflwsp(CIType {
            rdrs1,
            funct3,
            imm: Imm::new(uimm54276 as u32, 8),
        }))
        .into(),
        (OPCODE_C2, 0b011) if (xlen == Xlen::X64 || xlen == Xlen::X128) && rdrs1 != 0 => {
            RVC(Cldsp(CIType {
                rdrs1,
                funct3,
                imm: Imm::new(uimm54386 as u32, 9),
            }))
            .into()
        }
        (OPCODE_C2, 0b100) => match (ins12, rdrs1, rs2) {
            (false, _, 0) if rdrs1 != 0 => RVC(Cjr(CRType { rdrs1, rs2, funct4 })).into(),
            (false, _, _) if rdrs1 != 0 => RVC(Cmv(CRType { rdrs1, rs2, funct4 })).into(),
            (true, 0, 0) => RVC(Cebreak(CRType { rdrs1, rs2, funct4 })).into(),
            (true, _, 0) => RVC(Cjalr(CRType { rdrs1, rs2, funct4 })).into(),
            (true, _, _) if rdrs1 != 0 => RVC(Cadd(CRType { rdrs1, rs2, funct4 })).into(),
            _ => Err(())?,
        },
        (OPCODE_C2, 0b101) if xlen == Xlen::X32 || xlen == Xlen::X64 => RVC(Cfsdsp(CSSType {
            rs2,
            funct3,
            imm: Imm::new(uimm5386 as u32, 9),
        }))
        .into(),
        (OPCODE_C2, 0b101) if xlen == Xlen::X128 => RVC(Csqsp(CSSType {
            rs2,
            funct3,
            imm: Imm::new(uimm54_96 as u32, 10),
        }))
        .into(),
        (OPCODE_C2, 0b110) => RVC(Cswsp(CSSType {
            rs2,
            funct3,
            imm: Imm::new(uimm5276 as u32, 8),
        }))
        .into(),
        (OPCODE_C2, 0b111) if xlen == Xlen::X32 => RVC(Cfswsp(CSSType {
            rs2,
            funct3,
            imm: Imm::new(uimm5276 as u32, 8),
        }))
        .into(),
        (OPCODE_C2, 0b111) if xlen == Xlen::X64 || xlen == Xlen::X128 => RVC(Csdsp(CSSType {
            rs2,
            funct3,
            imm: Imm::new(uimm5386 as u32, 9),
        }))
        .into(),
        _ => Err(())?,
    };
    Ok(ans)
}

fn c_reg(regid: u8) -> u8 {
    regid + 8
}

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

fn resolve_u32(ins: u32, xlen: Xlen) -> core::result::Result<Instruction, ()> {
    use {self::RVZicsr::*, self::RV32I::*, self::RV64I::*, self::RVF::*};
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
                _ => Err(())?,
            },
            FUNCT3_OP_SLT if funct7 == 0 => Slt(r_type).into(),
            FUNCT3_OP_SLTU if funct7 == 0 => Sltu(r_type).into(),
            FUNCT3_OP_XOR if funct7 == 0 => Xor(r_type).into(),
            FUNCT3_OP_OR if funct7 == 0 => Or(r_type).into(),
            FUNCT3_OP_AND if funct7 == 0 => And(r_type).into(),
            FUNCT3_OP_SLL if funct7 == 0 && xlen == Xlen::X32 => RV32I::Sll(r_type).into(),
            FUNCT3_OP_SLL if funct7 & 0b1111110 == 0 && xlen == Xlen::X64 => {
                RV64I::Sll(r_type).into()
            }
            FUNCT3_OP_SRL_SRA => match funct7 {
                FUNCT7_OP_SRL if xlen == Xlen::X32 => RV32I::Srl(r_type).into(),
                FUNCT7_OP_SRA if xlen == Xlen::X32 => RV32I::Sra(r_type).into(),
                FUNCT7_OP_SRL if xlen == Xlen::X64 => RV64I::Srl(r_type).into(),
                FUNCT7_OP_SRA if xlen == Xlen::X64 => RV64I::Sra(r_type).into(),
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

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    RV32I(RV32I),
    RV64I(RV64I),
    RVC(RVC),
    RVZicsr(RVZicsr),
    RVF(RVF),
}

impl Instruction {
    pub fn disassembly(&self) -> String {
        match self {
            Self::RV32I(rv32i) => rv32i.to_string(),
            Self::RV64I(rv64i) => rv64i.to_string(),
            Self::RVC(rvc) => rvc.to_string(),
            Self::RVZicsr(rvzicsr) => rvzicsr.to_string(),
            Self::RVF(rvf) => rvf.to_string(),
        }
    }
}

impl From<RV32I> for Instruction {
    fn from(src: RV32I) -> Instruction {
        Instruction::RV32I(src)
    }
}

impl From<RV64I> for Instruction {
    fn from(src: RV64I) -> Instruction {
        Instruction::RV64I(src)
    }
}

impl From<RVC> for Instruction {
    fn from(src: RVC) -> Instruction {
        Instruction::RVC(src)
    }
}

impl From<RVZicsr> for Instruction {
    fn from(src: RVZicsr) -> Instruction {
        Instruction::RVZicsr(src)
    }
}

impl From<RVF> for Instruction {
    fn from(src: RVF) -> Instruction {
        Instruction::RVF(src)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum RV32I {
    Lui(UType),
    Auipc(UType),
    Jal(JType),
    Jalr(IType),

    Beq(BType),
    Bne(BType),
    Blt(BType),
    Bge(BType),
    Bltu(BType),
    Bgeu(BType),

    Lb(IType),
    Lh(IType),
    Lw(IType),
    Lbu(IType),
    Lhu(IType),
    Sb(SType),
    Sh(SType),
    Sw(SType),

    Addi(IType),
    Slti(IType),
    Sltiu(IType),
    Xori(IType),
    Ori(IType),
    Andi(IType),
    Slli(IType),
    Srli(IType),
    Srai(IType),

    Add(RType),
    Sub(RType),
    Sll(RType),
    Slt(RType),
    Sltu(RType),
    Xor(RType),
    Srl(RType),
    Sra(RType),
    Or(RType),
    And(RType),

    Fence(IType),
    Ecall(IType),
    Ebreak(IType),
}

impl RV32I {
    pub fn to_string(&self) -> String {
        match self {
            Self::Lui(u) => format!("lui {}, {:?}", to_register(u.rd), u.imm),
            Self::Auipc(u) => format!("auipc {}, {:?}", to_register(u.rd), u.imm),
            Self::Jal(j) => format!("jal {}, {:?}", to_register(j.rd), j.imm),
            Self::Jalr(i) => format!(
                "jalr {}, {:?}({})",
                to_register(i.rd),
                i.imm,
                to_register(i.rs1)
            ),

            Self::Beq(b) => format!(
                "beq {}, {}, {:?}",
                to_register(b.rs1),
                to_register(b.rs2),
                b.imm
            ),
            Self::Bne(b) => format!(
                "bne {}, {}, {:?}",
                to_register(b.rs1),
                to_register(b.rs2),
                b.imm
            ),
            Self::Blt(b) => format!(
                "blt {}, {}, {:?}",
                to_register(b.rs1),
                to_register(b.rs2),
                b.imm
            ),
            Self::Bge(b) => format!(
                "bge {}, {}, {:?}",
                to_register(b.rs1),
                to_register(b.rs2),
                b.imm
            ),
            Self::Bltu(b) => format!(
                "bltu {}, {}, {:?}",
                to_register(b.rs1),
                to_register(b.rs2),
                b.imm
            ),
            Self::Bgeu(b) => format!(
                "bgeu {}, {}, {:?}",
                to_register(b.rs1),
                to_register(b.rs2),
                b.imm
            ),

            Self::Lb(i) => format!(
                "lb {}, {:?}({})",
                to_register(i.rd),
                i.imm,
                to_register(i.rs1)
            ),
            Self::Lh(i) => format!(
                "lh {}, {:?}({})",
                to_register(i.rd),
                i.imm,
                to_register(i.rs1)
            ),
            Self::Lw(i) => format!(
                "lw {}, {:?}({})",
                to_register(i.rd),
                i.imm,
                to_register(i.rs1)
            ),
            Self::Lbu(i) => format!(
                "lbu {}, {:?}({})",
                to_register(i.rd),
                i.imm,
                to_register(i.rs1)
            ),
            Self::Lhu(i) => format!(
                "lhu {}, {:?}({})",
                to_register(i.rd),
                i.imm,
                to_register(i.rs1)
            ),

            Self::Sb(s) => format!(
                "sb {}, {:?}({})",
                to_register(s.rs2),
                s.imm,
                to_register(s.rs1)
            ),
            Self::Sh(s) => format!(
                "sh {}, {:?}({})",
                to_register(s.rs2),
                s.imm,
                to_register(s.rs1)
            ),
            Self::Sw(s) => format!(
                "sw {}, {:?}({})",
                to_register(s.rs2),
                s.imm,
                to_register(s.rs1)
            ),

            Self::Add(r) => format!(
                "add {}, {}, {}",
                to_register(r.rd),
                to_register(r.rs1),
                to_register(r.rs2)
            ),
            Self::Sub(r) => format!(
                "sub {}, {}, {}",
                to_register(r.rd),
                to_register(r.rs1),
                to_register(r.rs2)
            ),
            Self::Sll(r) => format!(
                "sll {}, {}, {}",
                to_register(r.rd),
                to_register(r.rs1),
                to_register(r.rs2)
            ),
            Self::Slt(r) => format!(
                "slt {}, {}, {}",
                to_register(r.rd),
                to_register(r.rs1),
                to_register(r.rs2)
            ),
            Self::Sltu(r) => format!(
                "sltu {}, {}, {}",
                to_register(r.rd),
                to_register(r.rs1),
                to_register(r.rs2)
            ),
            Self::Xor(r) => format!(
                "xor {}, {}, {}",
                to_register(r.rd),
                to_register(r.rs1),
                to_register(r.rs2)
            ),
            Self::Srl(r) => format!(
                "srl {}, {}, {}",
                to_register(r.rd),
                to_register(r.rs1),
                to_register(r.rs2)
            ),
            Self::Sra(r) => format!(
                "sra {}, {}, {}",
                to_register(r.rd),
                to_register(r.rs1),
                to_register(r.rs2)
            ),
            Self::Or(r) => format!(
                "or {}, {}, {}",
                to_register(r.rd),
                to_register(r.rs1),
                to_register(r.rs2)
            ),
            Self::And(r) => format!(
                "and {}, {}, {}",
                to_register(r.rd),
                to_register(r.rs1),
                to_register(r.rs2)
            ),

            Self::Addi(i) => format!(
                "addi {}, {}, {:?}",
                to_register(i.rd),
                to_register(i.rs1),
                i.imm
            ),
            Self::Slti(i) => format!(
                "slti {}, {}, {:?}",
                to_register(i.rd),
                to_register(i.rs1),
                i.imm
            ),
            Self::Sltiu(i) => format!(
                "sltiu {}, {}, {:?}",
                to_register(i.rd),
                to_register(i.rs1),
                i.imm
            ),
            Self::Xori(i) => format!(
                "xori {}, {}, {:?}",
                to_register(i.rd),
                to_register(i.rs1),
                i.imm
            ),
            Self::Ori(i) => format!(
                "ori {}, {}, {:?}",
                to_register(i.rd),
                to_register(i.rs1),
                i.imm
            ),
            Self::Andi(i) => format!(
                "andi {}, {}, {:?}",
                to_register(i.rd),
                to_register(i.rs1),
                i.imm
            ),
            Self::Slli(i) => format!(
                "slli {}, {}, {:?}",
                to_register(i.rd),
                to_register(i.rs1),
                i.imm
            ), // TODO shamt，这里先用imm代替
            Self::Srli(i) => format!(
                "srli {}, {}, {:?}",
                to_register(i.rd),
                to_register(i.rs1),
                i.imm
            ), // TODO shamt，这里先用imm代替
            Self::Srai(i) => format!(
                "srai {}, {}, {:?}",
                to_register(i.rd),
                to_register(i.rs1),
                i.imm
            ), // TODO shamt，这里先用imm代替

            Self::Fence(_) => format!("fence"),
            Self::Ecall(_) => format!("ecall"),
            Self::Ebreak(_) => format!("ebreak"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum RV64I {
    Lwu(IType),
    Ld(IType),
    Sd(SType),

    Sll(RType),
    Srl(RType),
    Sra(RType),

    Slli(IType),
    Srli(IType),
    Srai(IType),

    Addiw(IType),
    Slliw(IType),
    Srliw(IType),
    Sraiw(IType),

    Addw(RType),
    Subw(RType),
    Sllw(RType),
    Srlw(RType),
    Sraw(RType),
}

impl RV64I {
    pub fn to_string(&self) -> String {
        match self {
            Self::Lwu(i) => format!(
                "lwu {}, {:?}({})",
                to_register(i.rd),
                i.imm,
                to_register(i.rs1)
            ),
            Self::Ld(i) => format!(
                "ld {}, {:?}({})",
                to_register(i.rd),
                i.imm,
                to_register(i.rs1)
            ),
            Self::Sd(s) => format!(
                "sd {}, {:?}({})",
                to_register(s.rs2),
                s.imm,
                to_register(s.rs1)
            ),

            Self::Sll(r) => format!(
                "sll {}, {}, {}",
                to_register(r.rd),
                to_register(r.rs1),
                to_register(r.rs2)
            ),
            Self::Srl(r) => format!(
                "srl {}, {}, {}",
                to_register(r.rd),
                to_register(r.rs1),
                to_register(r.rs2)
            ),
            Self::Sra(r) => format!(
                "sra {}, {}, {}",
                to_register(r.rd),
                to_register(r.rs1),
                to_register(r.rs2)
            ),
            Self::Slli(i) => format!(
                "slli {}, {}, {:?}",
                to_register(i.rd),
                to_register(i.rs1),
                i.imm
            ),
            Self::Srli(i) => format!(
                "srli {}, {}, {:?}",
                to_register(i.rd),
                to_register(i.rs1),
                i.imm
            ),
            Self::Srai(i) => format!(
                "srai {}, {}, {:?}",
                to_register(i.rd),
                to_register(i.rs1),
                i.imm
            ),

            Self::Addiw(i) => format!(
                "addiw {}, {}, {:?}",
                to_register(i.rd),
                to_register(i.rs1),
                i.imm
            ),
            Self::Slliw(i) => format!(
                "slliw {}, {}, {:?}",
                to_register(i.rd),
                to_register(i.rs1),
                i.imm
            ),
            Self::Srliw(i) => format!(
                "srliw {}, {}, {:?}",
                to_register(i.rd),
                to_register(i.rs1),
                i.imm
            ),
            Self::Sraiw(i) => format!(
                "sraiw {}, {}, {:?}",
                to_register(i.rd),
                to_register(i.rs1),
                i.imm
            ),

            Self::Addw(r) => format!(
                "addw {}, {}, {}",
                to_register(r.rd),
                to_register(r.rs1),
                to_register(r.rs2)
            ),
            Self::Subw(r) => format!(
                "subw {}, {}, {}",
                to_register(r.rd),
                to_register(r.rs1),
                to_register(r.rs2)
            ),
            Self::Sllw(r) => format!(
                "sllw {}, {}, {}",
                to_register(r.rd),
                to_register(r.rs1),
                to_register(r.rs2)
            ),
            Self::Srlw(r) => format!(
                "srlw {}, {}, {}",
                to_register(r.rd),
                to_register(r.rs1),
                to_register(r.rs2)
            ),
            Self::Sraw(r) => format!(
                "sraw {}, {}, {}",
                to_register(r.rd),
                to_register(r.rs1),
                to_register(r.rs2)
            ),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct UType {
    pub rd: u8,
    pub imm: Imm,
}

#[derive(Debug, Clone, Copy)]
pub struct JType {
    pub rd: u8,
    pub imm: Imm,
}

#[derive(Debug, Clone, Copy)]
pub struct IType {
    pub rd: u8,
    pub rs1: u8,
    pub funct3: u8,
    pub imm: Imm,
}

#[derive(Debug, Clone, Copy)]
pub struct SType {
    pub rs1: u8,
    pub rs2: u8,
    pub funct3: u8,
    pub imm: Imm,
}

#[derive(Debug, Clone, Copy)]
pub struct BType {
    pub rs1: u8,
    pub rs2: u8,
    pub funct3: u8,
    pub imm: Imm,
}

#[derive(Debug, Clone, Copy)]
pub struct RType {
    pub rd: u8,
    pub rs1: u8,
    pub rs2: u8,
    pub funct3: u8,
    pub funct7: u8,
}

#[derive(Debug, Clone, Copy)]
pub enum RVC {
    Caddi4spn(CIWType),
    Cfld(CLType),
    Clq(CLType),
    Clw(CLType),
    Cflw(CLType),
    Cld(CLType),
    Cfsd(CSType),
    Csq(CSType),
    Csw(CSType),
    Cfsw(CSType),
    Csd(CSType),

    Cnop(CIType),
    Caddi(CIType),
    Cjal(CJType),
    Caddiw(CIType),
    Cli(CIType),
    Caddi16sp(CIType),
    Clui(CIType),
    Csrli(CIType),
    Csrli64(CIType),
    Csrai(CIType),
    Csrai64(CIType),
    Candi(CIType),
    Csub(CAType),
    Cxor(CAType),
    Cor(CAType),
    Cand(CAType),
    Csubw(CAType),
    Caddw(CAType),
    Cj(CJType),
    Cbeqz(CBType),
    Cbnez(CBType),

    Cslli(CIType),
    Cslli64(CIType),
    Cfldsp(CIType),
    Clqsp(CIType),
    Clwsp(CIType),
    Cflwsp(CIType),
    Cldsp(CIType),
    Cjr(CRType),
    Cmv(CRType),
    Cebreak(CRType),
    Cjalr(CRType),
    Cadd(CRType),
    Cfsdsp(CSSType),
    Csqsp(CSSType),
    Cswsp(CSSType),
    Cfswsp(CSSType),
    Csdsp(CSSType),
}

impl RVC {
    pub fn to_string(&self) -> String {
        match self {
            Self::Caddi4spn(ciw) => format!(
                "c.addi {}, {}, {:?}",
                to_register(ciw.rd),
                to_register(2),
                ciw.uimm
            ),
            Self::Cfld(cl) => format!(
                "c.fld {}, {:?}({})",
                to_register(cl.rd),
                cl.imm,
                to_register(cl.rs1)
            ),
            Self::Clq(cl) => format!(
                "c.lq {}, {:?}({})",
                to_register(cl.rd),
                cl.imm,
                to_register(cl.rs1)
            ),
            Self::Clw(cl) => format!(
                "c.lw {}, {:?}({})",
                to_register(cl.rd),
                cl.imm,
                to_register(cl.rs1)
            ),
            Self::Cflw(cl) => format!(
                "c.flw {}, {:?}({})",
                to_register(cl.rd),
                cl.imm,
                to_register(cl.rs1)
            ),
            Self::Cld(cl) => format!(
                "c.ld {}, {:?}({})",
                to_register(cl.rd),
                cl.imm,
                to_register(cl.rs1)
            ),
            Self::Cfsd(cs) => format!(
                "c.fsd {}, {:?}({})",
                to_register(cs.rs2),
                cs.imm,
                to_register(cs.rs1)
            ),
            Self::Csq(cs) => format!(
                "c.sq {}, {:?}({})",
                to_register(cs.rs2),
                cs.imm,
                to_register(cs.rs1)
            ),
            Self::Csw(cs) => format!(
                "c.sw {}, {:?}({})",
                to_register(cs.rs2),
                cs.imm,
                to_register(cs.rs1)
            ),
            Self::Cfsw(cs) => format!(
                "c.fsw {}, {:?}({})",
                to_register(cs.rs2),
                cs.imm,
                to_register(cs.rs1)
            ),
            Self::Csd(cs) => format!(
                "c.sd {}, {:?}({})",
                to_register(cs.rs2),
                cs.imm,
                to_register(cs.rs1)
            ),

            Self::Cnop(ci) => format!("c.nop"),
            Self::Caddi(ci) => format!(
                "c.addi {}, {}, {:?}",
                to_register(ci.rdrs1),
                to_register(ci.rdrs1),
                ci.imm
            ),
            Self::Cjal(cj) => format!("c.jal {},{:?}", to_register(0), cj.target),
            Self::Caddiw(ci) => format!(
                "c.addiw {}, {}, {:?}",
                to_register(ci.rdrs1),
                to_register(ci.rdrs1),
                ci.imm
            ),
            Self::Cli(ci) => format!(
                "c.li {}, {}, {:?}",
                to_register(ci.rdrs1),
                to_register(0),
                ci.imm
            ),
            Self::Caddi16sp(ci) => format!(
                "c.addi16sp {}, {}, {:?}",
                to_register(2),
                to_register(2),
                ci.imm
            ),
            Self::Clui(ci) => format!("c.lui {}, {:?}", to_register(ci.rdrs1), ci.imm),
            Self::Csrli(ci) => format!(
                "c.srli {}, {}, {:?}",
                to_register(ci.rdrs1),
                to_register(ci.rdrs1),
                ci.imm
            ),
            Self::Csrli64(ci) => format!(
                "c.srli64 {}, {}, {:?}",
                to_register(ci.rdrs1),
                to_register(ci.rdrs1),
                ci.imm
            ),
            Self::Csrai(ci) => format!(
                "c.srai {}, {}, {:?}",
                to_register(ci.rdrs1),
                to_register(ci.rdrs1),
                ci.imm,
            ),
            Self::Csrai64(ci) => format!(
                "c.srai64 {}, {}, {:?}",
                to_register(ci.rdrs1),
                to_register(ci.rdrs1),
                ci.imm,
            ),
            Self::Candi(ci) => format!(
                "c.andi {}, {}, {:?}",
                to_register(ci.rdrs1),
                to_register(ci.rdrs1),
                ci.imm,
            ),
            Self::Csub(ca) => format!(
                "c.sub {}, {}, {}",
                to_register(ca.rdrs1),
                to_register(ca.rdrs1),
                to_register(ca.rs2),
            ),
            Self::Cxor(ca) => format!(
                "c.xor {}, {}, {}",
                to_register(ca.rdrs1),
                to_register(ca.rdrs1),
                to_register(ca.rs2),
            ),
            Self::Cor(ca) => format!(
                "c.or {}, {}, {}",
                to_register(ca.rdrs1),
                to_register(ca.rdrs1),
                to_register(ca.rs2),
            ),
            Self::Cand(ca) => format!(
                "c.and {}, {}, {}",
                to_register(ca.rdrs1),
                to_register(ca.rdrs1),
                to_register(ca.rs2),
            ),
            Self::Csubw(ca) => format!(
                "c.subw {}, {}, {}",
                to_register(ca.rdrs1),
                to_register(ca.rdrs1),
                to_register(ca.rs2),
            ),
            Self::Caddw(ca) => format!(
                "c.addw {}, {}, {}",
                to_register(ca.rdrs1),
                to_register(ca.rdrs1),
                to_register(ca.rs2),
            ),
            Self::Cj(cj) => format!("c.j {}, {:?}", to_register(0), cj.target),
            Self::Cbeqz(cb) => format!(
                "c.beqz {}, {}, {:?}",
                to_register(cb.rs1),
                to_register(0),
                cb.off
            ),
            Self::Cbnez(cb) => format!(
                "c.bnez {}, {}, {:?}",
                to_register(cb.rs1),
                to_register(0),
                cb.off
            ),

            Self::Cslli(ci) => format!(
                "c.slli {}, {}, {:?}",
                to_register(ci.rdrs1),
                to_register(ci.rdrs1),
                ci.imm
            ),
            Self::Cslli64(ci) => format!(
                "c.slli64 {}, {}, {:?}",
                to_register(ci.rdrs1),
                to_register(ci.rdrs1),
                ci.imm
            ),
            Self::Cfldsp(ci) => format!(
                "c.fldsp {}, {:?}({})",
                to_register(ci.rdrs1),
                ci.imm,
                to_register(2)
            ),
            Self::Clqsp(ci) => format!(
                "c.lqsp {}, {:?}({})",
                to_register(ci.rdrs1),
                ci.imm,
                to_register(2)
            ),
            Self::Clwsp(ci) => format!(
                "c.lwsp {}, {:?}({})",
                to_register(ci.rdrs1),
                ci.imm,
                to_register(2)
            ),
            Self::Cflwsp(ci) => format!(
                "c.flwsp {}, {:?}({})",
                to_register(ci.rdrs1),
                ci.imm,
                to_register(2)
            ),
            Self::Cldsp(ci) => format!(
                "c.ldsp {}, {:?}({})",
                to_register(ci.rdrs1),
                ci.imm,
                to_register(2)
            ),
            Self::Cjr(cr) => format!("c.jr {}, 0({})", to_register(0), to_register(cr.rdrs1)),
            Self::Cmv(cr) => format!(
                "c.mv {}, {}, {}",
                to_register(cr.rdrs1),
                to_register(0),
                to_register(cr.rs2)
            ),
            Self::Cebreak(cr) => format!("c.ebreak"),
            Self::Cjalr(cr) => format!("c.jalr {}, 0({})", to_register(1), to_register(cr.rdrs1)),
            Self::Cadd(cr) => format!(
                "c.add {}, {}, {}",
                to_register(cr.rdrs1),
                to_register(cr.rdrs1),
                to_register(cr.rs2)
            ),
            Self::Cfsdsp(css) => format!(
                "c.fsdsp {}, {:?}({})",
                to_register(css.rs2),
                css.imm,
                to_register(2)
            ),
            Self::Csqsp(css) => format!(
                "c.sqsp {}, {:?}({})",
                to_register(css.rs2),
                css.imm,
                to_register(2)
            ),
            Self::Cswsp(css) => format!(
                "c.swsp {}, {:?}({})",
                to_register(css.rs2),
                css.imm,
                to_register(2)
            ),
            Self::Cfswsp(css) => format!(
                "c.fswsp {}, {:?}({})",
                to_register(css.rs2),
                css.imm,
                to_register(2)
            ),
            Self::Csdsp(css) => format!(
                "c.sdsp {}, {:?}({})",
                to_register(css.rs2),
                css.imm,
                to_register(2)
            ),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CRType {
    pub rdrs1: u8,
    pub rs2: u8,
    pub funct4: u8,
}

#[derive(Debug, Clone, Copy)]
pub struct CIType {
    pub rdrs1: u8,
    pub funct3: u8,
    pub imm: Imm,
}

#[derive(Debug, Clone, Copy)]
pub struct CSSType {
    pub rs2: u8,
    pub funct3: u8,
    pub imm: Imm,
}

#[derive(Debug, Clone, Copy)]
pub struct CIWType {
    pub rd: u8,
    pub funct3: u8,
    pub uimm: Uimm,
}

#[derive(Debug, Clone, Copy)]
pub struct CLType {
    pub rd: u8,
    pub rs1: u8,
    pub funct3: u8,
    pub imm: Imm,
}

#[derive(Debug, Clone, Copy)]
pub struct CSType {
    pub rs1: u8,
    pub rs2: u8,
    pub funct3: u8,
    pub imm: Imm,
}

#[derive(Debug, Clone, Copy)]
pub struct CAType {
    pub rdrs1: u8,
    pub rs2: u8,
    pub funct2: u8,
    pub funct6: u8,
}

#[derive(Debug, Clone, Copy)]
pub struct CBType {
    pub rs1: u8,
    pub funct3: u8,
    pub off: Imm,
}

#[derive(Debug, Clone, Copy)]
pub struct CJType {
    pub funct3: u8,
    pub target: Imm,
}

#[derive(Debug, Clone, Copy)]
pub enum RVZicsr {
    Csrrw(CsrRType),
    Csrrs(CsrRType),
    Csrrc(CsrRType),
    Csrrwi(CsrIType),
    Csrrsi(CsrIType),
    Csrrci(CsrIType),
}

impl RVZicsr {
    pub fn to_string(&self) -> String {
        // TODO
        format!("RVZicsr Instruction")
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CsrRType {
    pub rd: u8,
    pub rs1: u8,
    pub funct3: u8,
    pub csr: u16,
}
#[derive(Debug, Clone, Copy)]
pub struct CsrIType {
    pub rd: u8,
    pub uimm: Uimm,
    pub funct3: u8,
    pub csr: u16,
}

#[derive(Debug, Clone, Copy)]
pub enum RVF {
    // RV32F
    Flw(IType),
    Fsw(SType),
    Fmadds(R4Type),
    Fmsubs(R4Type),
    Fnmadds(R4Type),
    Fnmsubs(R4Type),
    Fadds(RType),
    Fsubs(RType),
    Fmuls(RType),
    Fdivs(RType),
    Fsqrts(RType),
    Fsgnjs(RType),
    Fsgnjns(RType),
    Fsgnjxs(RType),
    Fmins(RType),
    Fmaxs(RType),
    Fcvtws(RType),
    Fcvtwus(RType),
    Fmvxw(RType),
    Feqs(RType),
    Flts(RType),
    Fles(RType),
    Fclasss(RType),
    Fcvtsw(RType),
    Fcvtswu(RType),
    Fmvwx(RType),
    // RV64F
    Fcvtls(RType),
    Fcvtlus(RType),
    Fcvtsl(RType),
    Fcvtslu(RType),
}

impl RVF {
    pub fn to_string(&self) -> String {
        // TODO
        format!("RVF Instruction")
    }
}

#[derive(Debug, Clone, Copy)]
pub struct R4Type {
    pub rd: u8,
    pub rs1: u8,
    pub rs2: u8,
    pub rs3: u8,
    pub funct3: u8,
    pub funct2: u8,
}

fn to_register(ins: u8) -> String {
    match ins {
        0 => "zero".to_string(),
        1 => "ra".to_string(),
        2 => "sp".to_string(),
        3 => "gp".to_string(),
        4 => "tp".to_string(),
        5 => "t0".to_string(),
        6 => "t1".to_string(),
        7 => "t2".to_string(),
        8 => "s0".to_string(),
        9 => "s1".to_string(),
        10 => "a0".to_string(),
        11 => "a1".to_string(),
        12 => "a2".to_string(),
        13 => "a3".to_string(),
        14 => "a4".to_string(),
        15 => "a5".to_string(),
        16 => "a6".to_string(),
        17 => "a7".to_string(),
        18 => "s2".to_string(),
        19 => "s3".to_string(),
        20 => "s4".to_string(),
        21 => "s5".to_string(),
        22 => "s6".to_string(),
        23 => "s7".to_string(),
        24 => "s8".to_string(),
        25 => "s9".to_string(),
        26 => "s10".to_string(),
        27 => "s11".to_string(),
        28 => "t3".to_string(),
        29 => "t4".to_string(),
        30 => "t5".to_string(),
        31 => "t6".to_string(),
        _ => "unknown".to_string(), // For values outside the valid register range
    }
}
