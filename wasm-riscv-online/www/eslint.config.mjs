import js from '@eslint/js';

export default [
    js.configs.recommended,
    {
        languageOptions: {
            ecmaVersion: 'latest',
            sourceType: 'module',
            globals: {
                // 针对浏览器环境和 WebAssembly  
                window: 'readonly',
                document: 'readonly',
                console: 'readonly',
                WebAssembly: 'readonly'
            }
        },
        rules: {
            'no-unused-vars': 'warn',
            'no-console': 'off', // 开发阶段允许 console  
            'indent': ['error', 2],
            'quotes': ['error', 'single'],
            'semi': ['error', 'always']
        }
    },
    {
        // 忽略特定文件  
        ignores: [
            'node_modules/**',
            'dist/**',
            '*.min.js',
            'webpack.config.js',
            '../pkg/**'
        ]
    }
];