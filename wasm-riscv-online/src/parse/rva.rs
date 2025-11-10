use crate::asm::{RType, RV32A};
use crate::riscv::imm::Xlen;
use super::common::{parse_register};

// 尝试解析RV32A指令
pub fn try_parse(mnem: &str, ops: &[String], xlen: Xlen) -> Option<Result<crate::asm::Instruction, String>> {
    // 只处理RV32A指令
    if xlen != Xlen::X32 {
        return None;
    }

    let result = match mnem {
        "lr.w" => parse_lr_w(ops),
        "sc.w" => parse_sc_w(ops),
        "amoswap.w" => parse_amo_w(ops, |r| RV32A::Amoswapw(r)),
        "amoadd.w" => parse_amo_w(ops, |r| RV32A::Amoaddw(r)),
        "amoxor.w" => parse_amo_w(ops, |r| RV32A::Amoxorw(r)),
        "amoand.w" => parse_amo_w(ops, |r| RV32A::Amoandw(r)),
        "amoor.w" => parse_amo_w(ops, |r| RV32A::Amoorw(r)),
        "amomin.w" => parse_amo_w(ops, |r| RV32A::Amominw(r)),
        "amomax.w" => parse_amo_w(ops, |r| RV32A::Amomaxw(r)),
        "amominu.w" => parse_amo_w(ops, |r| RV32A::Amominuw(r)),
        "amomaxu.w" => parse_amo_w(ops, |r| RV32A::Amomaxuw(r)),
        _ => return None, // 不是RV32A指令
    };

    Some(result.map(|rv32a| crate::asm::Instruction::RV32A(rv32a)))
}

// 解析lr.w指令：lr.w rd, (rs1)
fn parse_lr_w(ops: &[String]) -> Result<RV32A, String> {
    if ops.len() != 2 {
        return Err("lr.w 指令需要2个操作数: rd, (rs1)".to_string());
    }

    let rd = parse_register(&ops[0])?;
    
    // 解析带括号的rs1
    let rs1_str = &ops[1];
    if !rs1_str.starts_with('(') || !rs1_str.ends_with(')') {
        return Err(format!("无效的内存操作数格式: {}", rs1_str));
    }
    let rs1_content = &rs1_str[1..rs1_str.len()-1];
    let rs1 = parse_register(rs1_content)?;

    // lr.w使用R-type格式，但rs2和funct3/funct7有特定值
    let r_type = RType {
        rd,
        rs1,
        rs2: 0, // lr.w中rs2通常为0
        funct3: 0b010, // 32位字操作
        funct7: (super::super::isa::FUNCT5_A_LR as u8) << 2,
    };

    Ok(RV32A::Lrw(r_type))
}

// 解析sc.w指令：sc.w rd, rs2, (rs1)
fn parse_sc_w(ops: &[String]) -> Result<RV32A, String> {
    if ops.len() != 3 {
        return Err("sc.w 指令需要3个操作数: rd, rs2, (rs1)".to_string());
    }

    let rd = parse_register(&ops[0])?;
    let rs2 = parse_register(&ops[1])?;
    
    // 解析带括号的rs1
    let rs1_str = &ops[2];
    if !rs1_str.starts_with('(') || !rs1_str.ends_with(')') {
        return Err(format!("无效的内存操作数格式: {}", rs1_str));
    }
    let rs1_content = &rs1_str[1..rs1_str.len()-1];
    let rs1 = parse_register(rs1_content)?;

    // sc.w使用R-type格式
    let r_type = RType {
        rd,
        rs1,
        rs2,
        funct3: 0b010, // 32位字操作
        funct7: (super::super::isa::FUNCT5_A_SC as u8) << 2,
    };

    Ok(RV32A::Scw(r_type))
}

// 解析AMO指令：amo*w rd, rs2, (rs1)
fn parse_amo_w<F>(ops: &[String], constructor: F) -> Result<RV32A, String>
where
    F: Fn(RType) -> RV32A,
{
    if ops.len() != 3 {
        return Err("AMO指令需要3个操作数: rd, rs2, (rs1)".to_string());
    }

    let rd = parse_register(&ops[0])?;
    let rs2 = parse_register(&ops[1])?;
    
    // 解析带括号的rs1
    let rs1_str = &ops[2];
    if !rs1_str.starts_with('(') || !rs1_str.ends_with(')') {
        return Err(format!("无效的内存操作数格式: {}", rs1_str));
    }
    let rs1_content = &rs1_str[1..rs1_str.len()-1];
    let rs1 = parse_register(rs1_content)?;

    // AMO指令使用R-type格式，但funct7会在构造函数中设置
    let r_type = RType {
        rd,
        rs1,
        rs2,
        funct3: 0b010, // 32位字操作
        funct7: 0, // 暂时设为0，构造函数会根据具体指令类型设置
    };

    Ok(constructor(r_type))
}
