use super::c_reg;
use crate::{asm::Instruction,asm::*, riscv::imm::{Imm, Uimm, Xlen}};

const OPCODE_C0: u16 = 0b00;
const OPCODE_C1: u16 = 0b01;
const OPCODE_C2: u16 = 0b10;

pub fn resolve_u16(ins: u16, xlen: Xlen) -> core::result::Result<Instruction, ()> {
    use crate::asm::RVC::*;
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
        (OPCODE_C0, 0b000) if nzuimm549623 != 0 => RVC::Caddi4spn(CIWType {
            rd: c_reg(r24_c),
            funct3,
            uimm: Uimm::new(nzuimm549623 as u32, 10),
        })
        .into(),
        (OPCODE_C0, 0b001) if xlen == Xlen::X32 || xlen == Xlen::X64 => RVC::Cfld(CLType {
            rd: c_reg(r24_c),
            rs1: c_reg(r79_c),
            funct3,
            imm: Imm::new(uimm5376 as u32, 8),
        })
        .into(),
        (OPCODE_C0, 0b001) if xlen == Xlen::X128 => RVC::Clq(CLType {
            rd: c_reg(r24_c),
            rs1: c_reg(r79_c),
            funct3,
            imm: Imm::new(uimm54876 as u32, 9),
        })
        .into(),
        (OPCODE_C0, 0b010) => RVC::Clw(CLType {
            rd: c_reg(r24_c),
            rs1: c_reg(r79_c),
            funct3,
            imm: Imm::new(uimm5326 as u32, 7),
        })
        .into(),
        (OPCODE_C0, 0b011) if xlen == Xlen::X32 => RVC::Cflw(CLType {
            rd: c_reg(r24_c),
            rs1: c_reg(r79_c),
            funct3,
            imm: Imm::new(uimm5326 as u32, 7),
        })
        .into(),
        (OPCODE_C0, 0b011) if xlen == Xlen::X64 || xlen == Xlen::X128 => RVC::Cld(CLType {
            rd: c_reg(r24_c),
            rs1: c_reg(r79_c),
            funct3,
            imm: Imm::new(uimm5376 as u32, 8),
        })
        .into(),
        (OPCODE_C0, 0b101) if xlen == Xlen::X32 || xlen == Xlen::X64 => RVC::Cfsd(CSType {
            rs1: c_reg(r79_c),
            rs2: c_reg(r24_c),
            funct3,
            imm: Imm::new(uimm5376 as u32, 8),
        })
        .into(),
        (OPCODE_C0, 0b101) if xlen == Xlen::X128 => RVC::Csq(CSType {
            rs1: c_reg(r79_c),
            rs2: c_reg(r24_c),
            funct3,
            imm: Imm::new(uimm54876 as u32, 9),
        })
        .into(),
        (OPCODE_C0, 0b110) => RVC::Csw(CSType {
            rs1: c_reg(r79_c),
            rs2: c_reg(r24_c),
            funct3,
            imm: Imm::new(uimm5326 as u32, 7),
        })
        .into(),
        (OPCODE_C0, 0b111) if xlen == Xlen::X32 =>RVC::Cfsw(CSType {
            rs1: c_reg(r79_c),
            rs2: c_reg(r24_c),
            funct3,
            imm: Imm::new(uimm5326 as u32, 7),
        })
        .into(),
        (OPCODE_C0, 0b111) if xlen == Xlen::X64 || xlen == Xlen::X128 => RVC::Csd(CSType {
            rs1: c_reg(r79_c),
            rs2: c_reg(r24_c),
            funct3,
            imm: Imm::new(uimm5376 as u32, 8),
        })
        .into(),
        (OPCODE_C1, 0b000) if rdrs1 == 0 => RVC::Cnop(CIType {
            rdrs1,
            funct3,
            imm: Imm::new(nzimm540 as u32, 6),
        })
        .into(),
        (OPCODE_C1, 0b000) if rdrs1 != 0 => RVC::Caddi(CIType {
            rdrs1,
            funct3,
            imm: Imm::new(nzimm540 as u32, 6),
        })
        .into(),
        (OPCODE_C1, 0b001) if xlen == Xlen::X32 => RVC::Cjal(CJType {
            funct3,
            target: Imm::new(imm114981067315 as u32, 12),
        })
        .into(),
        (OPCODE_C1, 0b001) if xlen == Xlen::X64 || xlen == Xlen::X128 => RVC::Caddiw(CIType {
            rdrs1,
            funct3,
            imm: Imm::new(imm540 as u32, 6),
        })
        .into(),
        (OPCODE_C1, 0b010) if rdrs1 != 0 => RVC::Cli(CIType {
            rdrs1,
            funct3,
            imm: Imm::new(imm540 as u32, 6),
        })
        .into(),
        (OPCODE_C1, 0b011) if rdrs1 == 2 => RVC::Caddi16sp(CIType {
            rdrs1,
            funct3,
            imm: Imm::new(nzimm946875 as u32, 10),
        })
        .into(),
        (OPCODE_C1, 0b011) if rdrs1 != 2 && rdrs1 != 0 && nzuimm171612 != 0 => RVC::Clui(CIType {
            rdrs1,
            funct3,
            imm: Imm::new(nzuimm171612, 18),
        })
        .into(),
        (OPCODE_C1, 0b100) => match (funct6 & 0b11, ins12, funct2) {
            (0b00, _, _) if !(xlen == Xlen::X32 && ins12) && nzuimm540 != 0 => RVC::Csrli(CIType {
                rdrs1: c_reg(r79_c),
                funct3,
                imm: Imm::new(nzuimm540 as u32, 6),
            })
            .into(),
            (0b00, _, _) if xlen == Xlen::X128 && nzuimm540 == 0 => RVC::Csrli64(CIType {
                rdrs1: c_reg(r79_c),
                funct3,
                imm: Imm::new(nzuimm540 as u32, 6),
            })
            .into(),
            (0b01, _, _) if !(xlen == Xlen::X32 && ins12) && nzuimm540 != 0 => RVC::Csrai(CIType {
                rdrs1: c_reg(r79_c),
                funct3,
                imm: Imm::new(nzuimm540 as u32, 6),
            })
            .into(),
            (0b01, _, _) if xlen == Xlen::X128 && nzuimm540 == 0 => RVC::Csrai64(CIType {
                rdrs1: c_reg(r79_c),
                funct3,
                imm: Imm::new(nzuimm540 as u32, 6),
            })
            .into(),
            (0b10, _, _) => RVC::Candi(CIType {
                rdrs1: c_reg(r79_c),
                funct3,
                imm: Imm::new(imm540 as u32, 6),
            })
            .into(),
            (0b11, false, 0b00) => RVC::Csub(CAType {
                rdrs1: c_reg(r79_c),
                rs2: c_reg(r24_c),
                funct2,
                funct6,
            })
            .into(),
            (0b11, false, 0b01) => RVC::Cxor(CAType {
                rdrs1: c_reg(r79_c),
                rs2: c_reg(r24_c),
                funct2,
                funct6,
            })
            .into(),
            (0b11, false, 0b10) => RVC::Cor(CAType {
                rdrs1: c_reg(r79_c),
                rs2: c_reg(r24_c),
                funct2,
                funct6,
            })
            .into(),
            (0b11, false, 0b11) => RVC::Cand(CAType {
                rdrs1: c_reg(r79_c),
                rs2: c_reg(r24_c),
                funct2,
                funct6,
            })
            .into(),
            (0b11, true, 0b00) if xlen == Xlen::X64 || xlen == Xlen::X128 => RVC::Csubw(CAType {
                rdrs1: c_reg(r79_c),
                rs2: c_reg(r24_c),
                funct2,
                funct6,
            })
            .into(),
            (0b11, true, 0b01) if xlen == Xlen::X64 || xlen == Xlen::X128 => RVC::Caddw(CAType {
                rdrs1: c_reg(r79_c),
                rs2: c_reg(r24_c),
                funct2,
                funct6,
            })
            .into(),
            _ => Err(())?,
        },
        (OPCODE_C1, 0b101) => RVC::Cj(CJType {
            funct3,
            target: Imm::new(imm114981067315 as u32, 12),
        })
        .into(),
        (OPCODE_C1, 0b110) => RVC::Cbeqz(CBType {
            rs1: c_reg(r79_c),
            funct3,
            off: Imm::new(imm84376215 as u32, 9),
        })
        .into(),
        (OPCODE_C1, 0b111) => RVC::Cbnez(CBType {
            rs1: c_reg(r79_c),
            funct3,
            off: Imm::new(imm84376215 as u32, 9),
        })
        .into(),
        (OPCODE_C2, 0b000) if rdrs1 != 0 && !(xlen == Xlen::X32 && ins12) && nzuimm540 != 0 => {
            RVC::Cslli(CIType {
                rdrs1,
                funct3,
                imm: Imm::new(nzuimm540 as u32, 6),
            })
            .into()
        }
        (OPCODE_C2, 0b000) if rdrs1 != 0 && xlen == Xlen::X128 && nzuimm540 == 0 => {
            RVC::Cslli64(CIType {
                rdrs1,
                funct3,
                imm: Imm::new(nzuimm540 as u32, 6),
            })
            .into()
        }
        (OPCODE_C2, 0b001) if xlen == Xlen::X32 || xlen == Xlen::X64 => RVC::Cfldsp(CIType {
            rdrs1,
            funct3,
            imm: Imm::new(uimm54386 as u32, 9),
        })
        .into(),
        (OPCODE_C2, 0b001) if xlen == Xlen::X128 && rdrs1 != 0 => RVC::Clqsp(CIType {
            rdrs1,
            funct3,
            imm: Imm::new(uimm5_4_96 as u32, 10),
        })
        .into(),
        (OPCODE_C2, 0b010) if rdrs1 != 0 => RVC::Clwsp(CIType {
            rdrs1,
            funct3,
            imm: Imm::new(uimm54276 as u32, 8),
        })
        .into(),
        (OPCODE_C2, 0b011) if xlen == Xlen::X32 => RVC::Cflwsp(CIType {
            rdrs1,
            funct3,
            imm: Imm::new(uimm54276 as u32, 8),
        })
        .into(),
        (OPCODE_C2, 0b011) if (xlen == Xlen::X64 || xlen == Xlen::X128) && rdrs1 != 0 => {
            RVC::Cldsp(CIType {
                rdrs1,
                funct3,
                imm: Imm::new(uimm54386 as u32, 9),
            })
            .into()
        }
        (OPCODE_C2, 0b100) => match (ins12, rdrs1, rs2) {
            (false, _, 0) if rdrs1 != 0 => RVC::Cjr(CRType { rdrs1, rs2, funct4 }).into(),
            (false, _, _) if rdrs1 != 0 => RVC::Cmv(CRType { rdrs1, rs2, funct4 }).into(),
            (true, 0, 0) => RVC::Cebreak(CRType { rdrs1, rs2, funct4 }).into(),
            (true, _, 0) => RVC::Cjalr(CRType { rdrs1, rs2, funct4 }).into(),
            (true, _, _) if rdrs1 != 0 => RVC::Cadd(CRType { rdrs1, rs2, funct4 }).into(),
            _ => Err(())?,
        },
        (OPCODE_C2, 0b101) if xlen == Xlen::X32 || xlen == Xlen::X64 => RVC::Cfsdsp(CSSType {
            rs2,
            funct3,
            imm: Imm::new(uimm5386 as u32, 9),
        })
        .into(),
        (OPCODE_C2, 0b101) if xlen == Xlen::X128 => RVC::Csqsp(CSSType {
            rs2,
            funct3,
            imm: Imm::new(uimm54_96 as u32, 10),
        })
        .into(),
        (OPCODE_C2, 0b110) => RVC::Cswsp(CSSType {
            rs2,
            funct3,
            imm: Imm::new(uimm5276 as u32, 8),
        })
        .into(),
        (OPCODE_C2, 0b111) if xlen == Xlen::X32 => RVC::Cfswsp(CSSType {
            rs2,
            funct3,
            imm: Imm::new(uimm5276 as u32, 8),
        })
        .into(),
        (OPCODE_C2, 0b111) if xlen == Xlen::X64 || xlen == Xlen::X128 => RVC::Csdsp(CSSType {
            rs2,
            funct3,
            imm: Imm::new(uimm5386 as u32, 9),
        })
        .into(),
        _ => Err(())?,
    };
    Ok(ans)
}
