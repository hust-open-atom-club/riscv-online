//! RV32I 32-bit instruction tests for RISC-V online disassembler  
  
#![cfg(target_arch = "wasm32")]  
  
extern crate wasm_bindgen_test;  
use wasm_bindgen_test::*;  
use wasm_riscv_online::disassemble;  
  
wasm_bindgen_test_configure!(run_in_browser);  
  
#[wasm_bindgen_test]  
fn test_rv32i_lui_instruction() {  
    // LUI x1, 0x12345 -> 0x12345037  
    let result = disassemble("12345037");  
    assert!(!result.starts_with("Error"));  
    assert!(result.contains("lui"));  
}  
  
#[wasm_bindgen_test]  
fn test_rv32i_auipc_instruction() {  
    // AUIPC x2, 0x12345 -> 0x12345117  
    let result = disassemble("12345117");  
    assert!(!result.starts_with("Error"));  
    assert!(result.contains("auipc"));  
}  
  
#[wasm_bindgen_test]  
fn test_rv32i_jal_instruction() {  
    // JAL x1, 8 -> 0x008000ef  
    let result = disassemble("008000ef");  
    assert!(!result.starts_with("Error"));  
    assert!(result.contains("jal"));  
}  
  
#[wasm_bindgen_test]  
fn test_rv32i_jalr_instruction() {  
    // JALR x1, x2, 4 -> 0x004100e7  
    let result = disassemble("004100e7");  
    assert!(!result.starts_with("Error"));  
    assert!(result.contains("jalr"));  
}  
  
#[wasm_bindgen_test]  
fn test_rv32i_branch_instructions() {  
    // BEQ x1, x2, 8 -> 0x00208463  
    let result = disassemble("00208463");  
    assert!(!result.starts_with("Error"));  
    assert!(result.contains("beq"));  
  
    // BNE x1, x2, 8 -> 0x00209463  
    let result = disassemble("00209463");  
    assert!(!result.starts_with("Error"));  
    assert!(result.contains("bne"));  
  
    // BLT x1, x2, 8 -> 0x0020c463  
    let result = disassemble("0020c463");  
    assert!(!result.starts_with("Error"));  
    assert!(result.contains("blt"));  
  
    // BGE x1, x2, 8 -> 0x0020d463  
    let result = disassemble("0020d463");  
    assert!(!result.starts_with("Error"));  
    assert!(result.contains("bge"));  
  
    // BLTU x1, x2, 8 -> 0x0020e463  
    let result = disassemble("0020e463");  
    assert!(!result.starts_with("Error"));  
    assert!(result.contains("bltu"));  
  
    // BGEU x1, x2, 8 -> 0x0020f463  
    let result = disassemble("0020f463");  
    assert!(!result.starts_with("Error"));  
    assert!(result.contains("bgeu"));  
}  
  
#[wasm_bindgen_test]  
fn test_rv32i_load_instructions() {  
    // LB x1, 4(x2) -> 0x00410083  
    let result = disassemble("00410083");  
    assert!(!result.starts_with("Error"));  
    assert!(result.contains("lb"));  
  
    // LH x1, 4(x2) -> 0x00411083  
    let result = disassemble("00411083");  
    assert!(!result.starts_with("Error"));  
    assert!(result.contains("lh"));  
  
    // LW x1, 4(x2) -> 0x00412083  
    let result = disassemble("00412083");  
    assert!(!result.starts_with("Error"));  
    assert!(result.contains("lw"));  
  
    // LBU x1, 4(x2) -> 0x00414083  
    let result = disassemble("00414083");  
    assert!(!result.starts_with("Error"));  
    assert!(result.contains("lbu"));  
  
    // LHU x1, 4(x2) -> 0x00415083  
    let result = disassemble("00415083");  
    assert!(!result.starts_with("Error"));  
    assert!(result.contains("lhu"));  
}  
  
#[wasm_bindgen_test]  
fn test_rv32i_store_instructions() {  
    // SB x1, 4(x2) -> 0x00110223  
    let result = disassemble("00110223");  
    assert!(!result.starts_with("Error"));  
    assert!(result.contains("sb"));  
  
    // SH x1, 4(x2) -> 0x00111223  
    let result = disassemble("00111223");  
    assert!(!result.starts_with("Error"));  
    assert!(result.contains("sh"));  
  
    // SW x1, 4(x2) -> 0x00112223  
    let result = disassemble("00112223");  
    assert!(!result.starts_with("Error"));  
    assert!(result.contains("sw"));  
}  
  
#[wasm_bindgen_test]  
fn test_rv32i_immediate_arithmetic() {  
    // ADDI x1, x2, 100 -> 0x06410093  
    let result = disassemble("06410093");  
    assert!(!result.starts_with("Error"));  
    assert!(result.contains("addi"));  
  
    // SLTI x1, x2, 100 -> 0x06412093  
    let result = disassemble("06412093");  
    assert!(!result.starts_with("Error"));  
    assert!(result.contains("slti"));  
  
    // SLTIU x1, x2, 100 -> 0x06413093  
    let result = disassemble("06413093");  
    assert!(!result.starts_with("Error"));  
    assert!(result.contains("sltiu"));  
  
    // XORI x1, x2, 100 -> 0x06414093  
    let result = disassemble("06414093");  
    assert!(!result.starts_with("Error"));  
    assert!(result.contains("xori"));  
  
    // ORI x1, x2, 100 -> 0x06416093  
    let result = disassemble("06416093");  
    assert!(!result.starts_with("Error"));  
    assert!(result.contains("ori"));  
  
    // ANDI x1, x2, 100 -> 0x06417093  
    let result = disassemble("06417093");  
    assert!(!result.starts_with("Error"));  
    assert!(result.contains("andi"));  
}  
  
#[wasm_bindgen_test]  
fn test_rv32i_shift_immediate() {  
    // SLLI x1, x2, 5 -> 0x00511093  
    let result = disassemble("00511093");  
    assert!(!result.starts_with("Error"));  
    assert!(result.contains("slli"));  
  
    // SRLI x1, x2, 5 -> 0x00515093  
    let result = disassemble("00515093");  
    assert!(!result.starts_with("Error"));  
    assert!(result.contains("srli"));  
  
    // SRAI x1, x2, 5 -> 0x40515093  
    let result = disassemble("40515093");  
    assert!(!result.starts_with("Error"));  
    assert!(result.contains("srai"));  
}  
  
#[wasm_bindgen_test]  
fn test_rv32i_register_arithmetic() {  
    // ADD x1, x2, x3 -> 0x003100b3  
    let result = disassemble("003100b3");  
    assert!(!result.starts_with("Error"));  
    assert!(result.contains("add"));  
  
    // SUB x1, x2, x3 -> 0x403100b3  
    let result = disassemble("403100b3");  
    assert!(!result.starts_with("Error"));  
    assert!(result.contains("sub"));  
  
    // SLT x1, x2, x3 -> 0x003120b3  
    let result = disassemble("003120b3");  
    assert!(!result.starts_with("Error"));  
    assert!(result.contains("slt"));  
  
    // SLTU x1, x2, x3 -> 0x003130b3  
    let result = disassemble("003130b3");  
    assert!(!result.starts_with("Error"));  
    assert!(result.contains("sltu"));  
  
    // XOR x1, x2, x3 -> 0x003140b3  
    let result = disassemble("003140b3");  
    assert!(!result.starts_with("Error"));  
    assert!(result.contains("xor"));  
  
    // OR x1, x2, x3 -> 0x003160b3  
    let result = disassemble("003160b3");  
    assert!(!result.starts_with("Error"));  
    assert!(result.contains("or"));  
  
    // AND x1, x2, x3 -> 0x003170b3  
    let result = disassemble("003170b3");  
    assert!(!result.starts_with("Error"));  
    assert!(result.contains("and"));  
}  
  
#[wasm_bindgen_test]  
fn test_rv32i_register_shifts() {  
    // SLL x1, x2, x3 -> 0x003110b3  
    let result = disassemble("003110b3");  
    assert!(!result.starts_with("Error"));  
    assert!(result.contains("sll"));  
  
    // SRL x1, x2, x3 -> 0x003150b3  
    let result = disassemble("003150b3");  
    assert!(!result.starts_with("Error"));  
    assert!(result.contains("srl"));  
  
    // SRA x1, x2, x3 -> 0x403150b3  
    let result = disassemble("403150b3");  
    assert!(!result.starts_with("Error"));  
    assert!(result.contains("sra"));  
}  
  
#[wasm_bindgen_test]  
fn test_rv32i_system_instructions() {  
    // ECALL -> 0x00000073  
    let result = disassemble("00000073");  
    assert!(!result.starts_with("Error"));  
    assert!(result.contains("ecall"));  
  
    // EBREAK -> 0x00100073  
    let result = disassemble("00100073");  
    assert!(!result.starts_with("Error"));  
    assert!(result.contains("ebreak"));  
  
    // FENCE -> 0x0000000f  
    let result = disassemble("0000000f");  
    assert!(!result.starts_with("Error"));  
    assert!(result.contains("fence"));  
}  
  
#[wasm_bindgen_test]  
fn test_rv32i_error_cases() {  
    // Test invalid opcode (should return error)  
    let result = disassemble("0000001b");  
    assert!(result.starts_with("Error"));  
  
    // Test invalid input  
    let result = disassemble("invalid_hex");  
    assert!(result.starts_with("Error"));  
}  
  
#[wasm_bindgen_test]  
fn test_rv32i_comprehensive_coverage() {  
    let test_cases = vec![  
        ("12345037", "lui"),    // LUI  
        ("12345017", "auipc"),  // AUIPC  
        ("008000ef", "jal"),    // JAL  
        ("004100e7", "jalr"),   // JALR  
        ("00410083", "lb"),     // LB  
        ("00411083", "lh"),     // LH  
        ("00412083", "lw"),     // LW  
        ("00414083", "lbu"),    // LBU  
        ("00415083", "lhu"),    // LHU  
        ("06410093", "addi"),   // ADDI  
        ("06412093", "slti"),   // SLTI  
        ("06413093", "sltiu"),  // SLTIU  
        ("06414093", "xori"),   // XORI  
        ("06416093", "ori"),    // ORI  
        ("06417093", "andi"),   // ANDI  
        ("00511093", "slli"),   // SLLI  
        ("00515093", "srli"),   // SRLI  
        ("40515093", "srai"),   // SRAI  
        ("00208463", "beq"),    // BEQ  
        ("00209463", "bne"),    // BNE  
        ("0020c463", "blt"),    // BLT  
        ("0020d463", "bge"),    // BGE  
        ("0020e463", "bltu"),   // BLTU  
        ("0020f463", "bgeu"),   // BGEU  
        ("00110223", "sb"),     // SB  
        ("00111223", "sh"),     // SH  
        ("00112223", "sw"),     // SW  
        ("003100b3", "add"),    // ADD  
        ("403100b3", "sub"),    // SUB  
        ("003110b3", "sll"),    // SLL  
        ("003120b3", "slt"),    // SLT  
        ("003130b3", "sltu"),   // SLTU  
        ("003140b3", "xor"),    // XOR  
        ("003150b3", "srl"),    // SRL  
        ("403150b3", "sra"),    // SRA  
        ("003160b3", "or"),     // OR  
        ("003170b3", "and"),    // AND  
        ("0000000f", "fence"),  // FENCE  
        ("00000073", "ecall"),  // ECALL  
        ("00100073", "ebreak"), // EBREAK  
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