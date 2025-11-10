use crate::asm::*;
use crate::riscv::imm::{Imm, Xlen};
use super::common::{parse_register, parse_int, imm_signed_bits, parse_mem_operand};

pub(crate) fn try_parse(mnem: &str, ops: &[String], xlen: Xlen) -> Option<Result<Instruction, String>> {
    // 检查是否为64位模式
    match xlen {
        Xlen::X64 | Xlen::X128 => {},
        _ => return None, // 仅在RV64/128中支持
    }

    match mnem {
        // FLD - 加载双精度浮点数
        "fld" => {
            if ops.len() != 2 { return Some(Err("用法: fld rd, imm(rs1)".into())); }
            let rd = match parse_register(&ops[0]) { Ok(v) => v, Err(e) => return Some(Err(e)) };
            let (imm_bits, rs1) = match parse_mem_operand(&ops[1]) { Ok(v) => v, Err(e) => return Some(Err(e)) };
            let i = IType { rd, rs1, funct3: 0, imm: Imm::new(imm_bits, 12) };
            Some(Ok(RV64D::Fld(i).into()))
        },
        
        // FSD - 存储双精度浮点数
        "fsd" => {
            if ops.len() != 2 { return Some(Err("用法: fsd rs2, imm(rs1)".into())); }
            let rs2 = match parse_register(&ops[0]) { Ok(v) => v, Err(e) => return Some(Err(e)) };
            let (imm_bits, rs1) = match parse_mem_operand(&ops[1]) { Ok(v) => v, Err(e) => return Some(Err(e)) };
            let s = SType { rs1, rs2, funct3: 0, imm: Imm::new(imm_bits, 12) };
            Some(Ok(RV64D::Fsd(s).into()))
        },
        
        // FADD.D - 双精度浮点数加法
        "fadd.d" => {
            if ops.len() != 3 { return Some(Err("用法: fadd.d rd, rs1, rs2".into())); }
            let rd = match parse_register(&ops[0]) { Ok(v) => v, Err(e) => return Some(Err(e)) };
            let rs1 = match parse_register(&ops[1]) { Ok(v) => v, Err(e) => return Some(Err(e)) };
            let rs2 = match parse_register(&ops[2]) { Ok(v) => v, Err(e) => return Some(Err(e)) };
            let r = RType { rd, rs1, rs2, funct3: 0, funct7: 0 };
            Some(Ok(RV64D::FaddD(r).into()))
        },
        
        // 其他RV64D指令的解析可以在这里添加
        
        _ => None,
    }
}