fn main() {
    println!("验证RV64D和RVB功能...");
    
    // 直接测试这些功能是否能正确处理
    println!("\nRV64D功能测试:");
    println!("fld指令 (64位):");
    let result = wasm_riscv_online::assemble_with_xlen("fld fa0, 0(x1)", 64);
    println!("结果: {}", result);
    
    println!("\nRVB功能测试:");
    println!("bset指令:");
    let result = wasm_riscv_online::assemble_with_xlen("bset x1, x2, x3", 32);
    println!("结果: {}", result);
}
