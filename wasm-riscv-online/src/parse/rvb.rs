use crate::asm::*;
use crate::riscv::imm::Imm;
use crate::riscv::imm::Xlen;
use super::common::{parse_register, parse_int};

pub(crate) fn try_parse(mnem: &str, ops: &[String], xlen: Xlen) -> Option<Result<Instruction, String>> {
    match mnem {
        // BSET - 位设置指令
        "bset" => {
            if ops.len() != 3 { return Some(Err("用法: bset rd, rs1, rs2".into())); }
            let rd = match parse_register(&ops[0]) { Ok(v) => v, Err(e) => return Some(Err(e)) };
            let rs1 = match parse_register(&ops[1]) { Ok(v) => v, Err(e) => return Some(Err(e)) };
            let rs2 = match parse_register(&ops[2]) { Ok(v) => v, Err(e) => return Some(Err(e)) };
            let r = RType { rd, rs1, rs2, funct3: 0, funct7: 0 };
            Some(Ok(RVB::Bset(r).into()))
        },
        
        // BEXT - 位提取指令
        "bext" => {
            if ops.len() != 3 { return Some(Err("用法: bext rd, rs1, rs2".into())); }
            let rd = match parse_register(&ops[0]) { Ok(v) => v, Err(e) => return Some(Err(e)) };
            let rs1 = match parse_register(&ops[1]) { Ok(v) => v, Err(e) => return Some(Err(e)) };
            let rs2 = match parse_register(&ops[2]) { Ok(v) => v, Err(e) => return Some(Err(e)) };
            let r = RType { rd, rs1, rs2, funct3: 0, funct7: 0 };
            Some(Ok(RVB::Bext(r).into()))
        },
        
        // BINV - 位反转指令
        "binv" => {
            if ops.len() != 3 { return Some(Err("用法: binv rd, rs1, rs2".into())); }
            let rd = match parse_register(&ops[0]) { Ok(v) => v, Err(e) => return Some(Err(e)) };
            let rs1 = match parse_register(&ops[1]) { Ok(v) => v, Err(e) => return Some(Err(e)) };
            let rs2 = match parse_register(&ops[2]) { Ok(v) => v, Err(e) => return Some(Err(e)) };
            let r = RType { rd, rs1, rs2, funct3: 0, funct7: 0 };
            Some(Ok(RVB::Binv(r).into()))
        },
        
        // BSETI - 立即数位设置指令
          "bseti" => {
              if ops.len() != 3 { return Some(Err("用法: bseti rd, rs1, imm".into())); }
              let rd = match parse_register(&ops[0]) { Ok(v) => v, Err(e) => return Some(Err(e)) };
              let rs1 = match parse_register(&ops[1]) { Ok(v) => v, Err(e) => return Some(Err(e)) };
              let imm_val = match parse_int(&ops[2]) { Ok(v) => v, Err(e) => return Some(Err(e)) };
              let i = IType { rd, rs1, imm: Imm::new(imm_val as u32, 5), funct3: 0 };
              Some(Ok(RVB::Bseti(i).into()))
          },
          
          // BEXTI - 立即数位提取指令
          "bexti" => {
              if ops.len() != 3 { return Some(Err("用法: bexti rd, rs1, imm".into())); }
              let rd = match parse_register(&ops[0]) { Ok(v) => v, Err(e) => return Some(Err(e)) };
              let rs1 = match parse_register(&ops[1]) { Ok(v) => v, Err(e) => return Some(Err(e)) };
              let imm_val = match parse_int(&ops[2]) { Ok(v) => v, Err(e) => return Some(Err(e)) };
              let i = IType { rd, rs1, imm: Imm::new(imm_val as u32, 5), funct3: 0 };
              Some(Ok(RVB::Bexti(i).into()))
          },
          
          // BINVI - 立即数位反转指令
          "binvi" => {
              if ops.len() != 3 { return Some(Err("用法: binvi rd, rs1, imm".into())); }
              let rd = match parse_register(&ops[0]) { Ok(v) => v, Err(e) => return Some(Err(e)) };
              let rs1 = match parse_register(&ops[1]) { Ok(v) => v, Err(e) => return Some(Err(e)) };
              let imm_val = match parse_int(&ops[2]) { Ok(v) => v, Err(e) => return Some(Err(e)) };
              let i = IType { rd, rs1, imm: Imm::new(imm_val as u32, 5), funct3: 0 };
              Some(Ok(RVB::Binvi(i).into()))
          },
        
        _ => None,
    }
}