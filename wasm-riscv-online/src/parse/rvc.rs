use crate::asm::*;
use crate::riscv::imm::{Imm, Uimm, Xlen};
use super::common::{parse_int, parse_mem_operand, parse_register, imm_signed_bits};

pub(crate) fn try_parse(mnem: &str, ops: &[String], xlen: Xlen) -> Option<Result<Instruction, String>> {
    match mnem {
        // CIW: c.addi4spn rd, uimm
        "c.addi4spn" => {
            if ops.len() != 2 { return Some(Err("用法: c.addi4spn rd, uimm".into())); }
            let rd = match parse_register(&ops[0]) { Ok(v) => v, Err(e) => return Some(Err(e)) };
            let uimm_val = match parse_int(&ops[1]) { Ok(v) => v, Err(e) => return Some(Err(e)) };
            if uimm_val <= 0 { return Some(Err("c.addi4spn 的 uimm 必须为正且非零".into())); }
            let u = uimm_val as u32;
            if (u & 0x3) != 0 { return Some(Err("c.addi4spn 的 uimm 必须按 4 字节对齐".into())); }
            if u >= (1 << 10) { return Some(Err("c.addi4spn 的 uimm 超出 10 位范围".into())); }
            let ciw = CIWType { rd, funct3: 0, uimm: Uimm::new(u, 10) };
            Some(Ok(RVC::Caddi4spn(ciw).into()))
        }
        // CJ: c.j imm (12-bit signed, PC-relative)
        "c.j" => {
            if ops.len() != 1 { return Some(Err("用法: c.j imm".into())); }
            let imm = match parse_int(&ops[0]) { Ok(v) => v, Err(e) => return Some(Err(e)) };
            let imm_bits = match imm_signed_bits(imm, 12) { Ok(v) => v, Err(e) => return Some(Err(e)) };
            let cj = CJType { funct3: 0, target: Imm::new(imm_bits, 12) };
            Some(Ok(RVC::Cj(cj).into()))
        }
        // CJ: c.jal imm (RV32 only)
        "c.jal" => {
            if ops.len() != 1 { return Some(Err("用法: c.jal imm".into())); }
            match xlen { Xlen::X32 => {}, _ => return Some(Err("c.jal 仅在 RV32 可用".into())) }
            let imm = match parse_int(&ops[0]) { Ok(v) => v, Err(e) => return Some(Err(e)) };
            let imm_bits = match imm_signed_bits(imm, 12) { Ok(v) => v, Err(e) => return Some(Err(e)) };
            let cj = CJType { funct3: 0, target: Imm::new(imm_bits, 12) };
            Some(Ok(RVC::Cjal(cj).into()))
        }
        // CB: c.beqz rs1c, off (9-bit signed)
        "c.beqz" => {
            if ops.len() != 2 { return Some(Err("用法: c.beqz rs1, off".into())); }
            let rs1 = match parse_register(&ops[0]) { Ok(v) => v, Err(e) => return Some(Err(e)) };
            let off = match parse_int(&ops[1]) { Ok(v) => v, Err(e) => return Some(Err(e)) };
            let off_bits = match imm_signed_bits(off, 9) { Ok(v) => v, Err(e) => return Some(Err(e)) };
            let cb = CBType { rs1, funct3: 0, off: Imm::new(off_bits, 9) };
            Some(Ok(RVC::Cbeqz(cb).into()))
        }
        // CB: c.bnez rs1c, off (9-bit signed)
        "c.bnez" => {
            if ops.len() != 2 { return Some(Err("用法: c.bnez rs1, off".into())); }
            let rs1 = match parse_register(&ops[0]) { Ok(v) => v, Err(e) => return Some(Err(e)) };
            let off = match parse_int(&ops[1]) { Ok(v) => v, Err(e) => return Some(Err(e)) };
            let off_bits = match imm_signed_bits(off, 9) { Ok(v) => v, Err(e) => return Some(Err(e)) };
            let cb = CBType { rs1, funct3: 0, off: Imm::new(off_bits, 9) };
            Some(Ok(RVC::Cbnez(cb).into()))
        }
        // CI-like: c.addi rd, imm
        "c.addi" => {
            if ops.len() != 2 { return Some(Err("用法: c.addi rd, imm".into())); }
            let rd = match parse_register(&ops[0]) { Ok(v) => v, Err(e) => return Some(Err(e)) };
            let imm = match parse_int(&ops[1]) { Ok(v) => v, Err(e) => return Some(Err(e)) };
            let imm_bits = match imm_signed_bits(imm, 6) { Ok(v) => v, Err(e) => return Some(Err(e)) };
            let ci = CIType { rdrs1: rd, funct3: 0, imm: Imm::new(imm_bits, 6) };
            Some(Ok(RVC::Caddi(ci).into()))
        }
        // c.addi16sp imm  (sp implicit)
        "c.addi16sp" => {
            if ops.len() != 1 { return Some(Err("用法: c.addi16sp imm".into())); }
            let imm = match parse_int(&ops[0]) { Ok(v) => v, Err(e) => return Some(Err(e)) };
            // 10-bit nzimm
            let imm_bits = match imm_signed_bits(imm, 10) { Ok(v) => v, Err(e) => return Some(Err(e)) };
            let ci = CIType { rdrs1: 2, funct3: 0, imm: Imm::new(imm_bits, 10) };
            Some(Ok(RVC::Caddi16sp(ci).into()))
        }
        // c.addiw rd, imm (RV64/128)
        "c.addiw" => {
            if ops.len() != 2 { return Some(Err("用法: c.addiw rd, imm".into())); }
            match xlen { Xlen::X64 | Xlen::X128 => {}, _ => return Some(Err("c.addiw 仅在 RV64/128 可用".into())) }
            let rd = match parse_register(&ops[0]) { Ok(v) => v, Err(e) => return Some(Err(e)) };
            let imm = match parse_int(&ops[1]) { Ok(v) => v, Err(e) => return Some(Err(e)) };
            let imm_bits = match imm_signed_bits(imm, 6) { Ok(v) => v, Err(e) => return Some(Err(e)) };
            let ci = CIType { rdrs1: rd, funct3: 0, imm: Imm::new(imm_bits, 6) };
            Some(Ok(RVC::Caddiw(ci).into()))
        }
        // c.slli rd, shamt
        "c.slli" => {
            if ops.len() != 2 { return Some(Err("用法: c.slli rd, shamt".into())); }
            let rd = match parse_register(&ops[0]) { Ok(v) => v, Err(e) => return Some(Err(e)) };
            let sh = match parse_int(&ops[1]) { Ok(v) => v as u32, Err(e) => return Some(Err(e)) };
            let ci = CIType { rdrs1: rd, funct3: 0, imm: Imm::new(sh, 6) };
            Some(Ok(RVC::Cslli(ci).into()))
        }
        // c.srli rd, shamt  (rd should be a compressed register; validated during encoding)
        "c.srli" => {
            if ops.len() != 2 { return Some(Err("用法: c.srli rd, shamt".into())); }
            let rd = match parse_register(&ops[0]) { Ok(v) => v, Err(e) => return Some(Err(e)) };
            let sh = match parse_int(&ops[1]) { Ok(v) => v as u32, Err(e) => return Some(Err(e)) };
            let ci = CIType { rdrs1: rd, funct3: 0, imm: Imm::new(sh, 6) };
            Some(Ok(RVC::Csrli(ci).into()))
        }
        // c.srai rd, shamt
        "c.srai" => {
            if ops.len() != 2 { return Some(Err("用法: c.srai rd, shamt".into())); }
            let rd = match parse_register(&ops[0]) { Ok(v) => v, Err(e) => return Some(Err(e)) };
            let sh = match parse_int(&ops[1]) { Ok(v) => v as u32, Err(e) => return Some(Err(e)) };
            let ci = CIType { rdrs1: rd, funct3: 0, imm: Imm::new(sh, 6) };
            Some(Ok(RVC::Csrai(ci).into()))
        }
        // c.andi rd, imm
        "c.andi" => {
            if ops.len() != 2 { return Some(Err("用法: c.andi rd, imm".into())); }
            let rd = match parse_register(&ops[0]) { Ok(v) => v, Err(e) => return Some(Err(e)) };
            let imm = match parse_int(&ops[1]) { Ok(v) => v, Err(e) => return Some(Err(e)) };
            let imm_bits = match imm_signed_bits(imm, 6) { Ok(v) => v, Err(e) => return Some(Err(e)) };
            let ci = CIType { rdrs1: rd, funct3: 0, imm: Imm::new(imm_bits, 6) };
            Some(Ok(RVC::Candi(ci).into()))
        }
        // CA logic: c.sub/xor/or/and  (rd', rs2')
        "c.sub" | "c.xor" | "c.or" | "c.and" => {
            if ops.len() != 2 { return Some(Err("用法: c.{sub|xor|or|and} rd, rs2".into())); }
            let rd = match parse_register(&ops[0]) { Ok(v) => v, Err(e) => return Some(Err(e)) };
            let rs2 = match parse_register(&ops[1]) { Ok(v) => v, Err(e) => return Some(Err(e)) };
            let ca = CAType { rdrs1: rd, rs2, funct2: 0, funct6: 0 };
            let inst = match mnem {
                "c.sub" => RVC::Csub(ca).into(),
                "c.xor" => RVC::Cxor(ca).into(),
                "c.or"  => RVC::Cor(ca).into(),
                "c.and" => RVC::Cand(ca).into(),
                _ => unreachable!(),
            };
            Some(Ok(inst))
        }
        // CA W-variants (RV64/128): c.subw/c.addw  (rd', rs2')
        "c.subw" | "c.addw" => {
            if ops.len() != 2 { return Some(Err("用法: c.{subw|addw} rd, rs2".into())); }
            match xlen { Xlen::X64 | Xlen::X128 => {}, _ => return Some(Err("该宽度变体仅在 RV64/128 可用".into())) }
            let rd = match parse_register(&ops[0]) { Ok(v) => v, Err(e) => return Some(Err(e)) };
            let rs2 = match parse_register(&ops[1]) { Ok(v) => v, Err(e) => return Some(Err(e)) };
            let ca = CAType { rdrs1: rd, rs2, funct2: 0, funct6: 0 };
            let inst = match mnem { "c.subw" => RVC::Csubw(ca).into(), _ => RVC::Caddw(ca).into() };
            Some(Ok(inst))
        }
        // CR group: c.mv rd, rs2 ; c.add rd, rs2 ; c.jr rs1 ; c.jalr rs1
        "c.mv" => {
            if ops.len() != 2 { return Some(Err("用法: c.mv rd, rs2".into())); }
            let rd = match parse_register(&ops[0]) { Ok(v) => v, Err(e) => return Some(Err(e)) };
            let rs2 = match parse_register(&ops[1]) { Ok(v) => v, Err(e) => return Some(Err(e)) };
            let cr = CRType { rdrs1: rd, rs2, funct4: 0 };
            Some(Ok(RVC::Cmv(cr).into()))
        }
        "c.add" => {
            if ops.len() != 2 { return Some(Err("用法: c.add rd, rs2".into())); }
            let rd = match parse_register(&ops[0]) { Ok(v) => v, Err(e) => return Some(Err(e)) };
            let rs2 = match parse_register(&ops[1]) { Ok(v) => v, Err(e) => return Some(Err(e)) };
            let cr = CRType { rdrs1: rd, rs2, funct4: 0 };
            Some(Ok(RVC::Cadd(cr).into()))
        }
        "c.jr" => {
            if ops.len() != 1 { return Some(Err("用法: c.jr rs1".into())); }
            let rs1 = match parse_register(&ops[0]) { Ok(v) => v, Err(e) => return Some(Err(e)) };
            let cr = CRType { rdrs1: rs1, rs2: 0, funct4: 0 };
            Some(Ok(RVC::Cjr(cr).into()))
        }
        "c.jalr" => {
            if ops.len() != 1 { return Some(Err("用法: c.jalr rs1".into())); }
            let rs1 = match parse_register(&ops[0]) { Ok(v) => v, Err(e) => return Some(Err(e)) };
            let cr = CRType { rdrs1: rs1, rs2: 0, funct4: 0 };
            Some(Ok(RVC::Cjalr(cr).into()))
        }
        // c.li rd, imm
        "c.li" => {
            if ops.len() != 2 { return Some(Err("用法: c.li rd, imm".into())); }
            let rd = match parse_register(&ops[0]) { Ok(v) => v, Err(e) => return Some(Err(e)) };
            let imm = match parse_int(&ops[1]) { Ok(v) => v, Err(e) => return Some(Err(e)) };
            let imm_bits = match imm_signed_bits(imm, 6) { Ok(v) => v, Err(e) => return Some(Err(e)) };
            let ci = CIType { rdrs1: rd, funct3: 0, imm: Imm::new(imm_bits, 6) };
            Some(Ok(RVC::Cli(ci).into()))
        }
        // c.nop
        "c.nop" => {
            let ci = CIType { rdrs1: 0, funct3: 0, imm: Imm::new(0, 6) };
            Some(Ok(RVC::Cnop(ci).into()))
        }
        // CL: c.lw rd, imm(rs1)
        "c.lw" => {
            if ops.len() != 2 { return Some(Err("用法: c.lw rd, imm(rs1)".into())); }
            let rd = match parse_register(&ops[0]) { Ok(v) => v, Err(e) => return Some(Err(e)) };
            let (imm_bits, rs1) = match parse_mem_operand(&ops[1]) { Ok(v) => v, Err(e) => return Some(Err(e)) };
            let cl = CLType { rd, rs1, funct3: 0, imm: Imm::new(imm_bits, 7) };
            Some(Ok(RVC::Clw(cl).into()))
        }
        // CS: c.sw rs2, imm(rs1)
        "c.sw" => {
            if ops.len() != 2 { return Some(Err("用法: c.sw rs2, imm(rs1)".into())); }
            let rs2 = match parse_register(&ops[0]) { Ok(v) => v, Err(e) => return Some(Err(e)) };
            let (imm_bits, rs1) = match parse_mem_operand(&ops[1]) { Ok(v) => v, Err(e) => return Some(Err(e)) };
            let cs = CSType { rs1, rs2, funct3: 0, imm: Imm::new(imm_bits, 7) };
            Some(Ok(RVC::Csw(cs).into()))
        }
        // C.LWSP: c.lwsp rd, imm(sp)
        "c.lwsp" => {
            if ops.len() != 2 { return Some(Err("用法: c.lwsp rd, imm(sp)".into())); }
            let rd = match parse_register(&ops[0]) { Ok(v) => v, Err(e) => return Some(Err(e)) };
            let (imm_bits, rs) = match parse_mem_operand(&ops[1]) { Ok(v) => v, Err(e) => return Some(Err(e)) };
            if rs != 2 { return Some(Err("c.lwsp 基址必须是 sp".into())); }
            let ci = CIType { rdrs1: rd, funct3: 0, imm: Imm::new(imm_bits, 8) };
            Some(Ok(RVC::Clwsp(ci).into()))
        }
        // C.SWSP: c.swsp rs2, imm(sp)
        "c.swsp" => {
            if ops.len() != 2 { return Some(Err("用法: c.swsp rs2, imm(sp)".into())); }
            let rs2 = match parse_register(&ops[0]) { Ok(v) => v, Err(e) => return Some(Err(e)) };
            let (imm_bits, rs) = match parse_mem_operand(&ops[1]) { Ok(v) => v, Err(e) => return Some(Err(e)) };
            if rs != 2 { return Some(Err("c.swsp 基址必须是 sp".into())); }
            let css = CSSType { rs2, funct3: 0, imm: Imm::new(imm_bits, 8) };
            Some(Ok(RVC::Cswsp(css).into()))
        }
        // RV64+: c.ld/c.sd/c.ldsp/c.sdsp
        "c.ld" => {
            if ops.len() != 2 { return Some(Err("用法: c.ld rd, imm(rs1)".into())); }
            match xlen { Xlen::X64 | Xlen::X128 => {}, _ => return Some(Err("c.ld 仅在 RV64/128 可用".into())) }
            let rd = match parse_register(&ops[0]) { Ok(v) => v, Err(e) => return Some(Err(e)) };
            let (imm_bits, rs1) = match parse_mem_operand(&ops[1]) { Ok(v) => v, Err(e) => return Some(Err(e)) };
            let cl = CLType { rd, rs1, funct3: 0, imm: Imm::new(imm_bits, 8) };
            Some(Ok(RVC::Cld(cl).into()))
        }
        "c.sd" => {
            if ops.len() != 2 { return Some(Err("用法: c.sd rs2, imm(rs1)".into())); }
            match xlen { Xlen::X64 | Xlen::X128 => {}, _ => return Some(Err("c.sd 仅在 RV64/128 可用".into())) }
            let rs2 = match parse_register(&ops[0]) { Ok(v) => v, Err(e) => return Some(Err(e)) };
            let (imm_bits, rs1) = match parse_mem_operand(&ops[1]) { Ok(v) => v, Err(e) => return Some(Err(e)) };
            let cs = CSType { rs1, rs2, funct3: 0, imm: Imm::new(imm_bits, 8) };
            Some(Ok(RVC::Csd(cs).into()))
        }
        "c.ldsp" => {
            if ops.len() != 2 { return Some(Err("用法: c.ldsp rd, imm(sp)".into())); }
            match xlen { Xlen::X64 | Xlen::X128 => {}, _ => return Some(Err("c.ldsp 仅在 RV64/128 可用".into())) }
            let rd = match parse_register(&ops[0]) { Ok(v) => v, Err(e) => return Some(Err(e)) };
            let (imm_bits, rs) = match parse_mem_operand(&ops[1]) { Ok(v) => v, Err(e) => return Some(Err(e)) };
            if rs != 2 { return Some(Err("c.ldsp 基址必须是 sp".into())); }
            let ci = CIType { rdrs1: rd, funct3: 0, imm: Imm::new(imm_bits, 9) };
            Some(Ok(RVC::Cldsp(ci).into()))
        }
        "c.sdsp" => {
            if ops.len() != 2 { return Some(Err("用法: c.sdsp rs2, imm(sp)".into())); }
            match xlen { Xlen::X64 | Xlen::X128 => {}, _ => return Some(Err("c.sdsp 仅在 RV64/128 可用".into())) }
            let rs2 = match parse_register(&ops[0]) { Ok(v) => v, Err(e) => return Some(Err(e)) };
            let (imm_bits, _rs) = match parse_mem_operand(&ops[1]) { Ok(v) => v, Err(e) => return Some(Err(e)) };
            let css = CSSType { rs2, funct3: 0, imm: Imm::new(imm_bits, 9) };
            Some(Ok(RVC::Csdsp(css).into()))
        }
        _ => None,
    }
}
