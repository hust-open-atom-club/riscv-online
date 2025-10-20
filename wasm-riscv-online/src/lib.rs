mod asm;
mod decode;
mod riscv;

use decode::{resolve_u16, resolve_u32};
use riscv::imm::Xlen;
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

#[wasm_bindgen]
pub fn disassemble_with_xlen(input: &str, xlen_bits: u32) -> String {
    let xlen = match xlen_bits {
        32 => Xlen::X32,
        64 => Xlen::X64,
        128 => Xlen::X128,
        _ => return format!("Error: invalid xlen {}, must be 32, 64, or 128", xlen_bits),
    };

    match input_to_u32(input) {
        Ok(value) => {
            if is_16_bit_instruction(value) {
                if value > 0xFFFF {
                    return format!("Error: invalid 16-bit instruction");
                }
                match resolve_u16((value & 0xFFFF) as u16, xlen) {
                    Ok(instruction) => instruction.disassembly(),
                    Err(_) => format!("Error: unsupported 16-bit instruction"),
                }
            } else {
                match resolve_u32(value, xlen) {
                    Ok(instruction) => instruction.disassembly(),
                    Err(_) => format!("Error: unsupported 32-bit instruction"),
                }
            }
        }
        Err(e) => format!("Error: invalid input: {}", e),
    }
}

#[wasm_bindgen]
pub fn disassemble_auto(input: &str) -> String {
    fn try_all<F>(f: F) -> Option<String>
    where
        F: Fn(Xlen) -> Result<String, ()>,
    {
        // Prefer 32-bit first, then 64, then 128
        if let Ok(s) = f(Xlen::X32) { return Some(s); }
        if let Ok(s) = f(Xlen::X64) { return Some(s); }
        if let Ok(s) = f(Xlen::X128) { return Some(s); }
        None
    }

    match input_to_u32(input) {
        Ok(value) => {
            if is_16_bit_instruction(value) {
                if value > 0xFFFF {
                    return format!("Error: invalid 16-bit instruction");
                }
                let try_decode = |xlen: Xlen| -> Result<String, ()> {
                    resolve_u16((value & 0xFFFF) as u16, xlen)
                        .map(|ins| ins.disassembly())
                };
                match try_all(try_decode) {
                    Some(s) => s,
                    None => "Error: unsupported 16-bit instruction".to_string(),
                }
            } else {
                let try_decode = |xlen: Xlen| -> Result<String, ()> {
                    resolve_u32(value, xlen).map(|ins| ins.disassembly())
                };
                match try_all(try_decode) {
                    Some(s) => s,
                    None => "Error: unsupported 32-bit instruction".to_string(),
                }
            }
        }
        Err(e) => format!("Error: invalid input: {}", e),
    }
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
