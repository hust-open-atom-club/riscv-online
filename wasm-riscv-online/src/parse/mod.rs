mod common;
mod rv_i;
mod system;
mod zicsr;
mod rvc;
mod rva;
use crate::asm::*;
use crate::riscv::imm::Xlen;
use self::common::{
    trim_comment,
    split_operands,
};

pub fn parse_line(line: &str, xlen: Xlen) -> Result<Instruction, String> {
    let raw = trim_comment(line).trim();
    if raw.is_empty() { return Err("空行".into()); }
    // split mnemonic and operands
    let mut parts = raw.split_whitespace();
    let mnem = parts.next().ok_or_else(|| "缺少助记符".to_string())?.to_lowercase();
    let rest = parts.collect::<Vec<_>>().join(" ");
    let ops = if rest.is_empty() { vec![] } else { split_operands(&rest) };

    // New modular dispatch (RVC -> Zicsr -> System -> RV A -> RV I)
    if let Some(res) = rvc::try_parse(&mnem, &ops, xlen)   { return res; }
    if let Some(res) = zicsr::try_parse(&mnem, &ops, xlen) { return res; }
    if let Some(res) = system::try_parse(&mnem, &ops, xlen){ return res; }
    if let Some(res) = rva::try_parse(&mnem, &ops, xlen)   { return res; }
    if let Some(res) = rv_i::try_parse(&mnem, &ops, xlen)  { return res; }
    // All known parsers failed; legacy fallback disabled. Return unsupported.
    return Err(format!("未支持的指令: {}", mnem));
}
