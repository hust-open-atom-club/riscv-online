import js from '@eslint/js';  
  
export default [  
  js.configs.recommended,  
  {  
    // 浏览器环境配置（用于 index.js, bootstrap.js 等）  
    files: ['*.js', '!.bin/**'],  
    languageOptions: {  
      ecmaVersion: 'latest',  
      sourceType: 'module',  
      globals: {  
        window: 'readonly',  
        document: 'readonly',  
        console: 'readonly',  
        WebAssembly: 'readonly',  
        setTimeout: 'readonly',  
        clearTimeout: 'readonly'  
      }  
    },  
    rules: {  
      'no-unused-vars': 'warn',  
      'no-console': 'off',  
      'indent': ['error', 2],  
      'quotes': ['error', 'single'],  
      'semi': ['error', 'always']  
    }  
  },  
  {  
    // Node.js 环境配置（用于 .bin/ 目录下的文件）  
    files: ['.bin/**/*.js'],  
    languageOptions: {  
      ecmaVersion: 'latest',  
      sourceType: 'commonjs',  
      globals: {  
        require: 'readonly',  
        process: 'readonly',  
        console: 'readonly',  
        Buffer: 'readonly',  
        __dirname: 'readonly',  
        __filename: 'readonly'  
      }  
    },  
    rules: {  
      'no-unused-vars': 'warn',  
      'no-console': 'off',  
      'indent': ['error', 2],  
      'quotes': ['error', 'single'],  
      'semi': ['error', 'always']  
    }  
  },  
  {  
    ignores: [  
      'node_modules/**',  
      'dist/**',  
      '*.min.js',  
      'webpack.config.js',  
      '../pkg/**'  
    ]  
  }  
];