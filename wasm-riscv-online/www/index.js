import * as wasm from 'wasm-riscv-online';
// wasm.greet();

try {
  const convertButton = document.getElementById('convertButton');
  const input = document.getElementById('input');
  const inputDisplay = document.getElementById('inputDisplay'); // 左侧框，显示输入的值
  const outputDisplay = document.getElementById('outputDisplay'); // 右侧框，显示结果的值
  // 为按钮添加点击事件监听器
  convertButton.addEventListener('click', () => {
    const inputValue = input.value;
    let wasmResult;
    // 显示正在处理的图标
    convertButton.classList.add('loading');
    inputDisplay.textContent = `Input: ${inputValue}`;
      // 调用 wasm 模块的 disassemble 函数进行反汇编
      wasmResult = wasm.disassemble(inputValue);
      outputDisplay.textContent = `Disassembly: ${wasmResult}`;
    // 处理完成后移除正在处理的图标
    setTimeout(() => {
      convertButton.classList.remove('loading');
    }, 1000); // 假设处理过程需要1秒
  });
} catch (error) {
  // 如果初始化过程中出现错误，将错误信息记录到控制台
  console.error('Failed to initialize and run the WebAssembly module:', error);
  // 同时在页面上显示错误信息
  document.getElementById('outputDisplay').textContent = 'Error loading the WebAssembly module.';
};