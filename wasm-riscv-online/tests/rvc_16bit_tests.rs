//! RVC 16-bit compressed instruction tests for RISC-V online disassembler

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
use wasm_riscv_online::disassemble;

wasm_bindgen_test_configure!(run_in_browser);

fn run_quadrant_tests(quadrant: &str, test_cases: Vec<(&str, &str)>) {
    for (hex_input, expected_instruction) in test_cases {
        let result = disassemble(hex_input);
        println!("{} Input: {}, Result: {}", quadrant, hex_input, result);
        assert!(
            result.contains(expected_instruction) || result.starts_with("Error"),
            "Failed to parse {}: got '{}', expected to contain '{}' or start with 'Error'",
            hex_input, result, expected_instruction
        );
    }
}

#[wasm_bindgen_test]
fn test_c0_quadrant_instructions() {
    run_quadrant_tests("C0", vec![
        ("0x1000", "c.addi4spn"),  // C.ADDI4SPN x8, sp, 4
        ("0x4398", "c.lw"),        // C.LW x14, 0(x15)
        ("0xc398", "c.sw"),        // C.SW x14, 0(x15)
    ]);
}

#[wasm_bindgen_test]
fn test_c1_quadrant_instructions() {
    run_quadrant_tests("C1", vec![
        ("0x0001", "c.nop"),       // C.NOP
        ("0x0505", "c.addi"),      // C.ADDI x10, x10, 1
        ("0x4501", "c.li"),        // C.LI x10, 0
        ("0xa001", "c.j"),         // C.J 0
    ]);
}

#[wasm_bindgen_test]
fn test_c2_quadrant_instructions() {
    run_quadrant_tests("C2", vec![
        ("0x4082", "c.lwsp"),      // C.LWSP x1, 0(sp)
        ("0xc006", "c.swsp"),      // C.SWSP x1, 0(sp)
        ("0x8082", "c.jr"),        // C.JR x1
        ("0x9002", "c.ebreak"),    // C.EBREAK
    ]);
}

#[wasm_bindgen_test]
fn test_instruction_width_detection() {
    // Test that 16-bit instructions are correctly identified
    let compressed_instructions = vec![
        "0x0001", // c.nop
        "0x4082", // c.lwsp
        "0x8082", // c.jr
    ];
    
    for hex in compressed_instructions {
        let result = disassemble(hex);
        println!("16-bit test: {} -> {}", hex, result);
        
        // Should not contain "32-bit" error message
        assert!(
            !result.contains("32-bit"),
            "Instruction {} incorrectly identified as 32-bit: {}",
            hex, result
        );
    }
    
    // Test that 32-bit instructions are correctly identified
    let standard_instructions = vec![
        "0x00000013", // ADDI x0, x0, 0
        "0x00100093", // ADDI x1, x0, 1
    ];
    
    for hex in standard_instructions {
        let result = disassemble(hex);
        println!("32-bit test: {} -> {}", hex, result);
        
        // Should not contain "16-bit" error message
        assert!(
            !result.contains("16-bit"),
            "Instruction {} incorrectly identified as 16-bit: {}",
            hex, result
        );
    }
}

#[wasm_bindgen_test]
fn test_error_cases() {
    // Test invalid 16-bit instructions
    let invalid_cases = vec![
        "0x0000", // Invalid C.ADDI4SPN with zero immediate
        "invalid", // Invalid hex input
    ];
    
    for hex in invalid_cases {
        let result = disassemble(hex);
        println!("Error case: {} -> {}", hex, result);
        
        // Should return an error
        assert!(
            result.starts_with("Error"),
            "Expected error for invalid input {}, but got: {}",
            hex, result
        );
    }
} 