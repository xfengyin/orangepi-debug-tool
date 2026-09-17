module.exports = {
  root: true,
  env: { browser: true, es2020: true },
  extends: [
    'eslint:recommended',
    'plugin:@typescript-eslint/recommended',
    'plugin:react-hooks/recommended',
    // 注：不再使用 'plugin:react-refresh/recommended'——该插件 0.4.26 起把
    // recommended 改为 flat config 结构（含 name/plugins 对象），ESLint 8 的
    // 传统 extends 语法会报 "Unexpected top-level property name"。
    // 其内容仅为 react-refresh/only-export-components 规则，已在下方 rules 中显式配置。
    'prettier',
  ],
  ignorePatterns: ['dist', '.eslintrc.cjs', 'src-tauri'],
  parser: '@typescript-eslint/parser',
  plugins: ['react-refresh'],
  rules: {
    'react-refresh/only-export-components': [
      'warn',
      { allowConstantExport: true },
    ],
    '@typescript-eslint/no-explicit-any': 'warn',
    '@typescript-eslint/no-unused-vars': ['error', { argsIgnorePattern: '^_' }],
  },
  settings: {
    react: {
      version: 'detect',
    },
  },
};