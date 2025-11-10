use wasm_riscv_online::{assemble_with_xlen};

#[test]
fn rv64d_instructions_test() {
    // RV64D浮点指令测试
    // fld指令 - 加载双精度浮点数
    let out32 = assemble_with_xlen("fld fa0, 0(x1)", 32);
    assert!(out32.trim().starts_with("Error:")); // 在32位模式下应该报错
    
    let out64 = assemble_with_xlen("fld fa0, 0(x1)", 64);
    assert_eq!(out64.trim(), "0x00008583"); // fld在64位模式下应该正确汇编
    
    // fsd指令 - 存储双精度浮点数
    let out32 = assemble_with_xlen("fsd fa0, 4(x1)", 32);
    assert!(out32.trim().starts_with("Error:")); // 在32位模式下应该报错
    
    let out64 = assemble_with_xlen("fsd fa0, 4(x1)", 64);
    assert_eq!(out64.trim(), "0x00a0a023"); // fsd在64位模式下应该正确汇编
    
    // fadd.d指令 - 双精度浮点数加法
    let out64 = assemble_with_xlen("fadd.d fa0, fa1, fa2", 64);
    assert_eq!(out64.trim(), "0x40a50553"); // fadd.d在64位模式下应该正确汇编
}

#[test]
fn rvb_instructions_test() {
    // RVB位操作指令测试
    // bset指令 - 位设置
    let out = assemble_with_xlen("bset x1, x2, x3", 32);
    assert_eq!(out.trim(), "0x023150b3"); // bset应该正确汇编
    
    // bext指令 - 位提取
    let out = assemble_with_xlen("bext x1, x2, x3", 32);
    assert_eq!(out.trim(), "0x023160b3"); // bext应该正确汇编
    
    // binv指令 - 位反转
    let out = assemble_with_xlen("binv x1, x2, x3", 32);
    assert_eq!(out.trim(), "0x023170b3"); // binv应该正确汇编
    
    // bseti指令 - 立即数位设置
    let out = assemble_with_xlen("bseti x1, x2, 5", 32);
    assert_eq!(out.trim(), "0x00515093"); // bseti应该正确汇编
    
    // bexti指令 - 立即数位提取
    let out = assemble_with_xlen("bexti x1, x2, 3", 32);
    assert_eq!(out.trim(), "0x00316093"); // bexti应该正确汇编
    
    // binvi指令 - 立即数位反转
    let out = assemble_with_xlen("binvi x1, x2, 7", 32);
    assert_eq!(out.trim(), "0x00717093"); // binvi应该正确汇编
}
