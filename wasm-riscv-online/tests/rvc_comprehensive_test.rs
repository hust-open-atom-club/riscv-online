//Comprehensive test suite for RISC-V 16-bit compressed instructions (RV-C).

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

// Use the `disassemble` function for testing, rather than calling `resolve_u16` directly.
use wasm_riscv_online::disassemble;

wasm_bindgen_test_configure!(run_in_browser);

/// Asserts that the disassembly does not start with "Error".
fn assert_disassembles_ok(hex: &str, instruction_name: &str) {
    let result = disassemble(hex);
    assert!(!result.starts_with("Error"),
            "Failed to decode {}: {}. Input was {}.", instruction_name, result, hex);
}

/// Asserts that the disassembly starts with "Error".
fn assert_disassembles_err(hex: &str, description: &str) {
    let result = disassemble(hex);
    assert!(result.starts_with("Error"),
            "Expected an error for {}: {}, but got: {}.", description, hex, result);
}

// ## C0 Quadrant Instructions ##
#[wasm_bindgen_test]
fn test_c0_quadrant() {
    // C.ADDI4SPN (funct3=000) - Corrected instruction encoding
    assert_disassembles_ok("0x1000", "c.addi4spn x8, sp, 4");
    
    // C.LW (funct3=010)
    assert_disassembles_ok("0x4398", "c.lw x14, 0(x15)");
    
    // C.SW (funct3=110)
    assert_disassembles_ok("0xc398", "c.sw x14, 0(x15)");
}

// ## C1 Quadrant Instructions ##
#[wasm_bindgen_test]
fn test_c1_quadrant() {
    // C.NOP (funct3=000, rd=0)
    assert_disassembles_ok("0x0001", "c.nop");
    
    // C.ADDI (funct3=000, rd!=0)
    assert_disassembles_ok("0x0505", "c.addi x10, x10, 1");
    
    // C.LI (funct3=010, rd!=0)
    assert_disassembles_ok("0x4501", "c.li x10, 0");
    
    // C.J (funct3=101)
    assert_disassembles_ok("0xa001", "c.j 0");
}

// ## C2 Quadrant Instructions ##
#[wasm_bindgen_test]
fn test_c2_quadrant() {
    // C.LWSP (funct3=010, rd!=0)
    assert_disassembles_ok("0x4082", "c.lwsp x1, 0(sp)");
    
    // C.SWSP (funct3=110)
    assert_disassembles_ok("0xc006", "c.swsp x1, 0(sp)");
    
    // C.JR (funct3=100, bit12=0, rs2=0, rd!=0)
    assert_disassembles_ok("0x8082", "c.jr x1");
    
    // C.EBREAK (funct3=100, bit12=1, rd=0, rs2=0)
    assert_disassembles_ok("0x9002", "c.ebreak");
}

// ## Test Error and Edge Cases ##
#[wasm_bindgen_test]
fn test_error_cases() {
    // Invalid instruction - C.ADDI4SPN with a zero immediate, which is disallowed.
    assert_disassembles_err("0x0000", "C.ADDI4SPN with nzuimm=0");
    
    // Invalid hex input string.
    assert_disassembles_err("invalid", "invalid hex input");
}

// ## Test Instruction Width Detection ##
#[wasm_bindgen_test]
fn test_instruction_width_detection() {
    // 16-bit instructions (compressed) should not be identified as 32-bit.
    // They do not end with the '11' bits pattern.
    let compressed_instructions = vec!["0x0001", "0x4082", "0x8000"];
    for hex in compressed_instructions {
        let result = disassemble(hex);
        assert!(!result.contains("32-bit"),
                "Instruction {} incorrectly identified as 32-bit: {}", hex, result);
    }
    
    // 32-bit instructions (standard) should not be identified as 16-bit.
    // They must end with the '11' bits pattern.
    let standard_instructions = vec!["0x00000013", "0x00100093"];
    for hex in standard_instructions {
        let result = disassemble(hex);
        assert!(!result.contains("16-bit"),
                "Instruction {} incorrectly identified as 16-bit: {}", hex, result);
    }
}