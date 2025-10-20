//! RV128A (.q) atomic instruction tests for RISC-V online disassembler

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
use wasm_riscv_online::disassemble_with_xlen;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_rv128a_load_reserved_store_conditional() {
    // LR.Q x1, (x2) -> funct5=00010, funct3=100, rd=1, rs1=2, rs2=0
    // [31:27]=00010 [26:25]=00 [24:20]=00000 [19:15]=00010 [14:12]=100 [11:7]=00001 [6:0]=0101111
    // 0001000000000001010000001010111111 = 0x1001402F
    let result = disassemble_with_xlen("1001402f", 128);
    assert!(!result.starts_with("Error"), "Failed to decode lr.q: {}", result);
    assert!(result.contains("lr.q"), "Expected 'lr.q' in '{}'", result);
    assert!(0 == 0);
    // SC.Q x1, x3, (x2) -> funct5=00011, funct3=100, rd=1, rs1=2, rs2=3
    // [31:27]=00011 [26:25]=00 [24:20]=00011 [19:15]=00010 [14:12]=100 [11:7]=00001 [6:0]=0101111
    // 0001100001100001010000001010111111 = 0x1831402F
    let result = disassemble_with_xlen("1831402f", 128);
    assert!(!result.starts_with("Error"), "Failed to decode sc.q: {}", result);
    assert!(result.contains("sc.q"), "Expected 'sc.q' in '{}'", result);
}

#[wasm_bindgen_test]
fn test_rv128a_atomic_swap() {
    // AMOSWAP.Q x1, x3, (x2) -> funct5=00001, funct3=100, rd=1, rs1=2, rs2=3
    // [31:27]=00001 [26:25]=00 [24:20]=00011 [19:15]=00010 [14:12]=100 [11:7]=00001 [6:0]=0101111
    // 0000100001100001010000001010111111 = 0x0831402F
    let result = disassemble_with_xlen("0831402f", 128);
    assert!(!result.starts_with("Error"), "Failed to decode amoswap.q: {}", result);
    assert!(result.contains("amoswap.q"), "Expected 'amoswap.q' in '{}'", result);
}

#[wasm_bindgen_test]
fn test_rv128a_atomic_arithmetic() {
    let test_cases = vec![
        ("0031402f", "amoadd.q"),   // AMOADD.Q x1, x3, (x2)
        ("2031402f", "amoxor.q"),   // AMOXOR.Q x1, x3, (x2)
        ("6031402f", "amoand.q"),   // AMOAND.Q x1, x3, (x2)
        ("4031402f", "amoor.q"),    // AMOOR.Q x1, x3, (x2)
        ("8031402f", "amomin.q"),   // AMOMIN.Q x1, x3, (x2)
        ("a031402f", "amomax.q"),   // AMOMAX.Q x1, x3, (x2)
        ("c031402f", "amominu.q"),  // AMOMINU.Q x1, x3, (x2)
        ("e031402f", "amomaxu.q"),  // AMOMAXU.Q x1, x3, (x2)
    ];

    for (encoding, mnemonic) in test_cases {
        let result = disassemble_with_xlen(encoding, 128);
        assert!(!result.starts_with("Error"), "Failed to decode {}: {}", mnemonic, result);
        assert!(result.contains(mnemonic), "Expected '{}' in '{}'", mnemonic, result);
    }
}

#[wasm_bindgen_test]
fn test_rv128a_with_acquire_release() {
    // LR.Q.AQ x1, (x2) -> funct5=00010, aq=1, rl=0, funct3=100, rd=1, rs1=2, rs2=0
    // [31:27]=00010 [26:25]=10 [24:20]=00000 [19:15]=00010 [14:12]=100 [11:7]=00001 [6:0]=0101111
    // 0001010000000001010000001010111111 = 0x1401402F
    let result = disassemble_with_xlen("1401402f", 128);
    assert!(!result.starts_with("Error"), "Failed to decode lr.q with acquire: {}", result);
    assert!(result.contains("lr.q"), "Expected 'lr.q' in '{}'", result);

    // SC.Q.RL x1, x3, (x2) -> funct5=00011, aq=0, rl=1, funct3=100, rd=1, rs1=2, rs2=3
    // [31:27]=00011 [26:25]=01 [24:20]=00011 [19:15]=00010 [14:12]=100 [11:7]=00001 [6:0]=0101111
    // 0001101001100001010000001010111111 = 0x1A31402F
    let result = disassemble_with_xlen("1a31402f", 128);
    assert!(!result.starts_with("Error"), "Failed to decode sc.q with release: {}", result);
    assert!(result.contains("sc.q"), "Expected 'sc.q' in '{}'", result);

    // AMOSWAP.Q.AQRL x1, x3, (x2) -> funct5=00001, aq=1, rl=1, funct3=100, rd=1, rs1=2, rs2=3
    // [31:27]=00001 [26:25]=11 [24:20]=00011 [19:15]=00010 [14:12]=100 [11:7]=00001 [6:0]=0101111
    // 0000111001100001010000001010111111 = 0x0E31402F
    let result = disassemble_with_xlen("0e31402f", 128);
    assert!(!result.starts_with("Error"), "Failed to decode amoswap.q with aq+rl: {}", result);
    assert!(result.contains("amoswap.q"), "Expected 'amoswap.q' in '{}'", result);
}

#[wasm_bindgen_test]
fn test_rv128a_different_registers() {
    let test_cases = vec![
        ("0050caaf", "amoadd.q"),   // AMOADD.Q x21, x5, (x1) - different registers (fixed funct3=100)
        ("100f402f", "lr.q"),       // LR.Q x1, (x30) - different base register
    ];

    for (encoding, mnemonic) in test_cases {
        let result = disassemble_with_xlen(encoding, 128);
        assert!(!result.starts_with("Error"), "Failed to decode {}: {}", mnemonic, result);
        assert!(result.contains(mnemonic), "Expected '{}' in '{}'", mnemonic, result);
    }
}

#[wasm_bindgen_test]
fn test_rv128a_comprehensive_coverage() {
    let test_cases = vec![
        ("1001402f", "lr.q"),       // LR.Q x1, (x2)
        ("1831402f", "sc.q"),       // SC.Q x1, x3, (x2)
        ("0831402f", "amoswap.q"),  // AMOSWAP.Q x1, x3, (x2)
        ("0031402f", "amoadd.q"),   // AMOADD.Q x1, x3, (x2)
        ("2031402f", "amoxor.q"),   // AMOXOR.Q x1, x3, (x2)
        ("6031402f", "amoand.q"),   // AMOAND.Q x1, x3, (x2)
        ("4031402f", "amoor.q"),    // AMOOR.Q x1, x3, (x2)
        ("8031402f", "amomin.q"),   // AMOMIN.Q x1, x3, (x2)
        ("a031402f", "amomax.q"),   // AMOMAX.Q x1, x3, (x2)
        ("c031402f", "amominu.q"),  // AMOMINU.Q x1, x3, (x2)
        ("e031402f", "amomaxu.q"),  // AMOMAXU.Q x1, x3, (x2)
    ];

    for (hex_input, expected_mnemonic) in test_cases {
        let result = disassemble_with_xlen(hex_input, 128);
        assert!(!result.starts_with("Error"), 
                "Failed to decode instruction: {}", hex_input);
        assert!(result.contains(expected_mnemonic), 
                "Expected '{}' in disassembly '{}' for instruction {}", 
                expected_mnemonic, result, hex_input);
    }
}

#[wasm_bindgen_test]
fn test_rv128a_error_cases() {
    let result = disassemble_with_xlen("f831402f", 128); // Invalid funct5=11111
    assert!(result.starts_with("Error"), "Expected error for invalid funct5");

    let result = disassemble_with_xlen("0031502f", 128); // Invalid funct3=101
    assert!(result.starts_with("Error"), "Expected error for invalid funct3");

    let result = disassemble_with_xlen("invalid_hex", 128);
    assert!(result.starts_with("Error"), "Expected error for invalid hex input");
}