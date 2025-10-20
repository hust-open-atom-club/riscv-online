//! RV32A atomic instruction tests for RISC-V online disassembler

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
use wasm_riscv_online::disassemble;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_rv32a_load_reserved_store_conditional() {
    // LR.W x1, (x2) -> 0x100120AF
    let result = disassemble("100120af");
    assert!(!result.starts_with("Error"), "Failed to decode lr.w: {}", result);
    assert!(result.contains("lr.w"), "Expected 'lr.w' in '{}'", result);

    // SC.W x1, x3, (x2) -> 0x183120AF
    let result = disassemble("183120af");
    assert!(!result.starts_with("Error"), "Failed to decode sc.w: {}", result);
    assert!(result.contains("sc.w"), "Expected 'sc.w' in '{}'", result);
}

#[wasm_bindgen_test]
fn test_rv32a_atomic_swap() {
    // AMOSWAP.W x1, x3, (x2) -> 0x083120AF
    let result = disassemble("083120af");
    assert!(!result.starts_with("Error"), "Failed to decode amoswap.w: {}", result);
    assert!(result.contains("amoswap.w"), "Expected 'amoswap.w' in '{}'", result);
}

#[wasm_bindgen_test]
fn test_rv32a_atomic_arithmetic() {
    let test_cases = vec![
        ("003120af", "amoadd.w"),   // AMOADD.W x1, x3, (x2)
        ("203120af", "amoxor.w"),   // AMOXOR.W x1, x3, (x2)
        ("603120af", "amoand.w"),   // AMOAND.W x1, x3, (x2)
        ("403120af", "amoor.w"),    // AMOOR.W x1, x3, (x2)
        ("803120af", "amomin.w"),   // AMOMIN.W x1, x3, (x2)
        ("a03120af", "amomax.w"),   // AMOMAX.W x1, x3, (x2)
        ("c03120af", "amominu.w"),  // AMOMINU.W x1, x3, (x2)
        ("e03120af", "amomaxu.w"),  // AMOMAXU.W x1, x3, (x2)
    ];

    for (encoding, mnemonic) in test_cases {
        let result = disassemble(encoding);
        assert!(!result.starts_with("Error"), "Failed to decode {}: {}", mnemonic, result);
        assert!(result.contains(mnemonic), "Expected '{}' in '{}'", mnemonic, result);
    }
}

#[wasm_bindgen_test]
fn test_rv32a_with_acquire_release() {
    // LR.W.AQ x1, (x2) -> 0x140120AF
    let result = disassemble("140120af");
    assert!(!result.starts_with("Error"), "Failed to decode lr.w with acquire: {}", result);
    assert!(result.contains("lr.w"), "Expected 'lr.w' in '{}'", result);

    // SC.W.RL x1, x3, (x2) -> 0x1A3120AF
    let result = disassemble("1a3120af");
    assert!(!result.starts_with("Error"), "Failed to decode sc.w with release: {}", result);
    assert!(result.contains("sc.w"), "Expected 'sc.w' in '{}'", result);

    // AMOSWAP.W.AQRL x1, x3, (x2) -> 0x0E3120AF
    let result = disassemble("0e3120af");
    assert!(!result.starts_with("Error"), "Failed to decode amoswap.w with aq+rl: {}", result);
    assert!(result.contains("amoswap.w"), "Expected 'amoswap.w' in '{}'", result);
}

#[wasm_bindgen_test]
fn test_rv32a_different_registers() {
    let test_cases = vec![
        ("0050aaaf", "amoadd.w"),   // AMOADD.W x21, x5, (x1) - different registers
        ("100f20af", "lr.w"),       // LR.W x1, (x30) - different base register
    ];

    for (encoding, mnemonic) in test_cases {
        let result = disassemble(encoding);
        assert!(!result.starts_with("Error"), "Failed to decode {}: {}", mnemonic, result);
        assert!(result.contains(mnemonic), "Expected '{}' in '{}'", mnemonic, result);
    }
}

#[wasm_bindgen_test]
fn test_rv32a_comprehensive_coverage() {
    let test_cases = vec![
        ("100120af", "lr.w"),       // LR.W x1, (x2)
        ("183120af", "sc.w"),       // SC.W x1, x3, (x2)
        ("083120af", "amoswap.w"),  // AMOSWAP.W x1, x3, (x2)
        ("003120af", "amoadd.w"),   // AMOADD.W x1, x3, (x2)
        ("203120af", "amoxor.w"),   // AMOXOR.W x1, x3, (x2)
        ("603120af", "amoand.w"),   // AMOAND.W x1, x3, (x2)
        ("403120af", "amoor.w"),    // AMOOR.W x1, x3, (x2)
        ("803120af", "amomin.w"),   // AMOMIN.W x1, x3, (x2)
        ("a03120af", "amomax.w"),   // AMOMAX.W x1, x3, (x2)
        ("c03120af", "amominu.w"),  // AMOMINU.W x1, x3, (x2)
        ("e03120af", "amomaxu.w"),  // AMOMAXU.W x1, x3, (x2)
    ];

    for (hex_input, expected_mnemonic) in test_cases {
        let result = disassemble(hex_input);
        assert!(!result.starts_with("Error"), 
                "Failed to decode instruction: {}", hex_input);
        assert!(result.contains(expected_mnemonic), 
                "Expected '{}' in disassembly '{}' for instruction {}", 
                expected_mnemonic, result, hex_input);
    }
}

#[wasm_bindgen_test]
fn test_rv32a_error_cases() {
    let result = disassemble("f83120af"); // Invalid funct5=11111
    assert!(result.starts_with("Error"), "Expected error for invalid funct5");

    // FIXME: This test case is incorrect - funct3=0b011 is actually valid for RV64A
    // The instruction 003130af decodes as:
    // - funct5=00000 (AMOADD), funct3=011 (RV64A .d width), rd=1, rs1=2, rs2=3
    // - This is a valid amoadd.d instruction in RV64A extension
    // - When RV64A support was added, this "invalid" test became a valid instruction
    // - Should use a truly invalid funct3 value (like 0b101 or 0b111) for error testing
    // let result = disassemble("003130af"); // Invalid funct3=011
    // assert!(result.starts_with("Error"), "Expected error for invalid funct3");

    let result = disassemble("invalid_hex");
    assert!(result.starts_with("Error"), "Expected error for invalid hex input");
}