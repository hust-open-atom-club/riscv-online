import * as wasm from 'wasm-riscv-online';

const PROCESSING_DELAY_MS = 500;
// Hoisted reusable regex patterns
const RE_LINE_BREAKS = /\r\n/g;
const RE_BYTE_STREAM = /^([0-9a-fA-F]{2}(\s+|$))+$/;
const RE_HEX_LINE = /^(0x|0X)?[0-9a-fA-F]+$/;
const RE_TWO_HEX = /^[0-9a-fA-F]{2}$/;

// Global state  
let isProcessing = false;
let keyboardShortcutsVisible = false;

try {
    // DOM element references  
    const convertButton = document.getElementById('convertButton');
    const clearButton = document.getElementById('clearButton');
    const copyButton = document.getElementById('copyButton');
    const input = document.getElementById('input');
    const xlenSelect = document.getElementById('xlenSelect');
    const modeSelect = document.getElementById('modeSelect');
    const inputDisplay = document.getElementById('inputDisplay');
    const outputDisplay = document.getElementById('outputDisplay');
    const inputStatus = document.getElementById('inputStatus');
    const errorMessage = document.getElementById('errorMessage');
    const keyboardShortcuts = document.getElementById('keyboardShortcuts');

    // Input validation (disassemble - hex)  
    function validateHexInput(value) {
        const trimmed = value.trim();
        if (!trimmed) {
            return { valid: false, message: '请输入内容', type: 'warning' };
        }

        // 判断是否为010字节流
        if (RE_BYTE_STREAM.test(trimmed.replace(RE_LINE_BREAKS, '\n').split('\n').map(l => l.trim()).join(' '))) {
            return { valid: true, message: '字节流格式', type: 'valid', format: 'byteStream' };
        }

        // Otherwise, validate each non-empty line as a hex string
        const lines = value.split('\n').filter(line => line.trim());

        for (let i = 0; i < lines.length; i++) {
            const cleanLine = lines[i].trim();
            if (!RE_HEX_LINE.test(cleanLine)) {
                return { valid: false, message: '包含无效的十六进制格式', type: 'error', format: null };
            }
        }

        return { valid: true, message: `${lines.length} 条指令`, type: 'valid', format: 'hex' };
    }

    // Input validation (assemble - text)
    function validateAsmInput(value) {
        const lines = value.split('\n').map(l => l.trim()).filter(Boolean);
        if (lines.length === 0) {
            return { valid: false, message: '请输入内容', type: 'warning' };
        }
        return { valid: true, message: `${lines.length} 条指令`, type: 'valid', format: 'asm' };
    }

    // Update input status UI  
    function updateInputStatus(validation) {
        inputStatus.textContent = validation.message;
        inputStatus.className = `input-status ${validation.type}`;

        input.className = validation.valid ? 'valid' : (validation.type === 'error' ? 'error' : '');
    }

    // Show error message  
    function showError(message) {
        errorMessage.textContent = message;
        errorMessage.classList.add('show');
        setTimeout(() => {
            errorMessage.classList.remove('show');
        }, 5000);
    }
    
    // Parse 010 Editor byte stream into per-line hex instructions
    function parseByteStream(value) {
        // 规范化空白并分割字节 token（每个 token 应为两位十六进制）
        const tokens = value.replace(RE_LINE_BREAKS, '\n').split(/\s+/).filter(Boolean);
        if (tokens.length === 0) return [];

        for (let i = 0; i < tokens.length; i++) {
            const t = tokens[i];
            if (!RE_TWO_HEX.test(t)) {
                throw new Error(`非法字节 token: "${t}"`);
            }
        }

        const lower = tokens.map(t => t.toLowerCase());
        const instructions = [];
        let i = 0;
        while (i < lower.length) {
            if (i + 1 >= lower.length) {
                throw new Error('指令不完整，剩余字节不足');
            }
            const b0 = lower[i];     // low byte (little-endian)
            const b1 = lower[i + 1]; // high byte (little-endian)
            const value16 = (parseInt(b0, 16)) | (parseInt(b1, 16) << 8);

            // If the lowest two bits are 11 => 32-bit instruction (read two more bytes)
            if ((value16 & 0x3) === 0x3) {
                if (i + 3 >= lower.length) {
                    throw new Error('指令不完整，剩余字节不足');
                }
                const b2 = lower[i + 2];
                const b3 = lower[i + 3];
                // Build big-endian hex string for display and processing (b3 b2 b1 b0)
                const hexBE = (b3 + b2 + b1 + b0).toLowerCase();
                instructions.push('0x' + hexBE.padStart(8, '0'));
                i += 4;
            } else {
                // 16-bit instruction, big-endian order b1 b0
                const hexBE = (b1 + b0).toLowerCase();
                instructions.push('0x' + hexBE.padStart(4, '0'));
                i += 2;
            }
        }
        return instructions;
    }

    // Disassemble using WASM by XLEN mode
    function disassembleByMode(formattedHex) {
        const mode = xlenSelect ? xlenSelect.value : 'auto';
        if (mode === 'auto') {
            return wasm.disassemble_auto(formattedHex);
        }
        const xlen = parseInt(mode, 10);
        return wasm.disassemble_with_xlen(formattedHex, xlen);
    }

    // Assemble using WASM by XLEN mode (whole text)
    function assembleByModeWhole(text) {
        const mode = xlenSelect ? xlenSelect.value : 'auto';
        if (mode === 'auto') {
            return wasm.assemble_auto(text);
        }
        const xlen = parseInt(mode, 10);
        return wasm.assemble_with_xlen(text, xlen);
    }

    // Process a single instruction  
    function processSingleInstruction(hexValue) {
        // Remove 0x prefix  
        if (hexValue.startsWith("0x") || hexValue.startsWith("0X")) {
            hexValue = hexValue.slice(2);
        }

        // Pad to even length  
        if (hexValue.length % 2 !== 0) {
            hexValue = '0' + hexValue;
        }

        // Convert to binary to determine instruction length  
        const binaryStr = parseInt(hexValue, 16).toString(2).padStart(32, '0');

        let formattedHexValue;
        if (binaryStr.endsWith('11')) {
            // 32-bit instruction  
            hexValue = hexValue.padStart(8, '0');
            formattedHexValue = '0x' + hexValue;
        } else {
            // 16-bit instruction  
            hexValue = hexValue.padStart(4, '0');
            formattedHexValue = '0x' + hexValue;
        }

        return {
            formatted: formattedHexValue,
            result: disassembleByMode(formattedHexValue)
        };
    }

    // Syntax highlighting  
    function highlightAssembly(text) {
        if (text.startsWith('Error:')) {
            return `<span class="assembly-error">${text}</span>`;
        }

        // Basic syntax highlighting  
        return text
            .replace(/\b(add|sub|mul|div|addi|subi|lw|sw|beq|bne|jal|jalr|nop|ret|li|fld|fsd|fadd\.d|binv|bset|bseti|bexti|binvi|andn|orn|xnor|ror|rori|rorw|roriw)\b/gi,
                '<span class="assembly-instruction">$1</span>')
            .replace(/\b(x[0-9]+|zero|ra|sp|gp|tp|t[0-6]|s[0-9]+|a[0-7]|f[0-9]+|ft[0-6]|fs[0-9]+|fa[0-7])\b/g,
                '<span class="assembly-register">$1</span>')
            .replace(/\b(-?0x[0-9a-fA-F]+|-?[0-9]+)\b/g,
                '<span class="assembly-immediate">$1</span>');
    }

    // Main conversion handler (branch by mode)  
    function handleConversion() {
        if (isProcessing) return;
        const mode = modeSelect ? modeSelect.value : 'disassemble';
        let inputValue = input.value.trim();
        const validation = mode === 'assemble' ? validateAsmInput(inputValue) : validateHexInput(inputValue);
        if (!validation.valid) { showError(validation.message); return; }
        if (mode === 'disassemble' && validation.format === 'byteStream') {
            try {
                const instructions = parseByteStream(inputValue);
                inputValue = instructions.join('\n');
            } catch (err) {
                showError(`${err.message}`);
                return;
            }
        }

        isProcessing = true;
        convertButton.disabled = true;
        convertButton.classList.add('loading');

        try {
            if (mode === 'disassemble') {
                const lines = inputValue.split('\n').filter(line => line.trim());
                const results = [];
                const inputs = [];
                for (let line of lines) {
                    const cleanLine = line.trim();
                    try {
                        const result = processSingleInstruction(cleanLine);
                        inputs.push(result.formatted);
                        results.push(result.result);
                    } catch (error) {
                        inputs.push(cleanLine);
                        results.push(`Error: ${error.message}`);
                    }
                }
                inputDisplay.innerHTML = inputs.map(input => `<div style="margin: 2px 0;">${input}</div>`).join('');
                outputDisplay.innerHTML = results.map(result => `<div style="margin: 2px 0;" class="assembly-output">${highlightAssembly(result)}</div>`).join('');
            } else { // assemble
                const outText = assembleByModeWhole(inputValue);
                const inLines = inputValue.split('\n').filter(l => l.trim());
                const outLines = outText.split('\n');
                inputDisplay.innerHTML = inLines.map(l => `<div style="margin: 2px 0;">${l}</div>`).join('');
                outputDisplay.innerHTML = outLines.map(r => `<div style="margin: 2px 0;" class="assembly-output">${r}</div>`).join('');
            }

        } catch (error) {
            showError(`处理失败：${error.message}`);
            console.error('Conversion error:', error);
        } finally {
            setTimeout(() => {
                isProcessing = false;
                convertButton.disabled = false;
                convertButton.classList.remove('loading');
            }, PROCESSING_DELAY_MS);
        }
    }

    // Clear input  
    function handleClear() {
        input.value = '';
        inputDisplay.innerHTML = '<div style="color: #666; font-style: italic;">输入的机器码将显示在这里...</div>';
        outputDisplay.innerHTML = '<div style="color: #666; font-style: italic;">反汇编结果将显示在这里...</div>';
        inputStatus.textContent = '';
        inputStatus.className = 'input-status';
        input.className = '';
        input.focus();
    }

    // Copy result  
    function handleCopy() {
        if (!outputDisplay.textContent.trim() ||
            outputDisplay.textContent.includes('将显示在这里')) {
            showError('暂无结果可复制');
            return;
        }
        const outputText = outputDisplay.textContent;
        if (outputText && !outputText.includes('反汇编结果将显示在这里')) {
            navigator.clipboard.writeText(outputText).then(() => {
                const originalText = copyButton.innerHTML;
                copyButton.innerHTML = '<i class="fas fa-check"></i><span>已复制</span>';
                setTimeout(() => {
                    copyButton.innerHTML = originalText;
                }, 2000);
            }).catch(err => {
                showError('复制失败，请手动选择文本复制');
            });
        }
    }

    // Event listeners  
    convertButton.addEventListener('click', handleConversion);
    clearButton.addEventListener('click', handleClear);
    copyButton.addEventListener('click', handleCopy);

    // Debounce timer
    let inputDebounceTimer = null;

    // Live input validation (300 ms debounce)
    input.addEventListener('input', () => {
        clearTimeout(inputDebounceTimer);          // cancel previous timer
        inputDebounceTimer = setTimeout(() => {    // start a new timer
            const mode = modeSelect ? modeSelect.value : 'disassemble';
            const validation = mode === 'assemble' ? validateAsmInput(input.value) : validateHexInput(input.value);
            updateInputStatus(validation);
        }, 300);  // execute only if no input within 300 ms
    });

    // Update placeholders and button text when mode changes
    function updateModeUI() {
        const mode = modeSelect ? modeSelect.value : 'disassemble';
        const buttonSpan = convertButton.querySelector('span');
        if (mode === 'assemble') {
            input.placeholder = '请输入 RISC-V 汇编，每行一条...\n\n示例：\naddi a0, a0, 1\nld x1, 0(x2)';
            buttonSpan.textContent = '转换/汇编';
            inputDisplay.innerHTML = '<div style="color: #666; font-style: italic;">输入的汇编将显示在这里...</div>';
            outputDisplay.innerHTML = '<div style="color: #666; font-style: italic;">机器码将显示在这里...</div>';
        } else {
            input.placeholder = '请输入十六进制机器码，支持多行输入...\n\n示例：\n0x00000013\n0x00100093\n0x00208233';
            buttonSpan.textContent = '转换/反汇编';
            inputDisplay.innerHTML = '<div style="color: #666; font-style: italic;">输入的机器码将显示在这里...</div>';
            outputDisplay.innerHTML = '<div style="color: #666; font-style: italic;">反汇编结果将显示在这里...</div>';
        }
        // Re-validate
        const validation = mode === 'assemble' ? validateAsmInput(input.value) : validateHexInput(input.value);
        updateInputStatus(validation);
    }
    if (modeSelect) {
        modeSelect.addEventListener('change', updateModeUI);
        updateModeUI();
    }

    // Keyboard shortcuts  
    document.addEventListener('keydown', (event) => {
        // Ctrl+Enter: perform conversion  
        if (event.ctrlKey && event.key === 'Enter') {
            event.preventDefault();
            handleConversion();
        }

        // Esc: clear input  
        if (event.key === 'Escape') {
            event.preventDefault();
            handleClear();
        }

        // F1: toggle shortcuts hint  
        if (event.key === 'F1') {
            event.preventDefault();
            keyboardShortcutsVisible = !keyboardShortcutsVisible;
            keyboardShortcuts.classList.toggle('show', keyboardShortcutsVisible);
        }
    });

    // Focus input on initialization  
    input.focus();

} catch (error) {
    console.error('Failed to initialize and run the WebAssembly module:', error);
    document.getElementById('outputDisplay').innerHTML =
        '<span class="assembly-error">Error loading the WebAssembly module.</span>';
}

// Global functions for HTML calls  
window.toggleHelp = function () {
    const helpContent = document.getElementById('helpContent');
    helpContent.classList.toggle('show');
};

window.loadExample = function (example) {
    const input = document.getElementById('input');
    input.value = example;
    input.focus();

    // Trigger input validation  
    const event = new Event('input', { bubbles: true });
    input.dispatchEvent(event);
};