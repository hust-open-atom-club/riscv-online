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
    let hexValue = inputValue;

    // 检查并补齐输入的十六进制数
    if (hexValue.startsWith('0x') || hexValue.startsWith('0X')) {
      hexValue = hexValue.slice(2);
    }

    // 如果输入值不是偶数位，前面补 0
    if (hexValue.length % 2 !== 0) {
      hexValue = '0' + hexValue;
    }

    // 转换为二进制字符串
    const binaryStr = parseInt(hexValue, 16).toString(2).padStart(32, '0');

    // 判断指令位长
    let formattedHexValue;
    if (binaryStr.endsWith('11')) {
      // 32 位指令，确保长度为 8 个字符
      hexValue = hexValue.padStart(8, '0');
      formattedHexValue = '0x' + hexValue;
    } else {
      // 16 位指令，确保长度为 4 个字符
      hexValue = hexValue.padStart(4, '0');
      formattedHexValue = '0x' + hexValue;
    }

    let wasmResult;
    // 显示正在处理的图标
    convertButton.classList.add('loading');
    inputDisplay.textContent = `Input: ${formattedHexValue}`;

    // 调用 wasm 模块的 disassemble 函数进行反汇编
    wasmResult = wasm.disassemble(formattedHexValue);
    outputDisplay.textContent = `Disassembly: ${wasmResult}`;

    // 处理完成后移除正在处理的图标
    setTimeout(() => {
      convertButton.classList.remove('loading');
    }, 1000); // 假设处理过程需要 1 秒
  });
} catch (error) {
  // 如果初始化过程中出现错误，将错误信息记录到控制台
  console.error('Failed to initialize and run the WebAssembly module:', error);
  // 同时在页面上显示错误信息
  document.getElementById('outputDisplay').textContent = 'Error loading the WebAssembly module.';
};

