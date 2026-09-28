/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  darkMode: 'class',
  theme: {
    extend: {
      // 与 src/theme/tokens.ts 保持一致（唯一权威源为 tokens.ts，此处为 Tailwind 镜像）
      colors: {
        brand: {
          DEFAULT: '#ff6b35',
          hover: '#ff8050',
          active: '#e85a24',
        },
        neutral: {
          bg: '#0b0c0e',
          surface: '#141518',
          'surface-alt': '#1b1c21',
          inset: '#0f1013',
          'text-primary': '#ededf0',
          'text-secondary': '#a2a6ad',
          'text-muted': '#6e737c',
        },
        success: '#3fc97f',
        warning: '#e9a23b',
        danger: '#f2555a',
        info: '#5ea2f7',
      },
      fontFamily: {
        sans: ['Inter', 'system-ui', '-apple-system', 'Segoe UI', 'sans-serif'],
        mono: ['JetBrains Mono', 'SF Mono', 'Menlo', 'Consolas', 'monospace'],
      },
      borderRadius: {
        DEFAULT: '10px',
      },
    },
  },
  plugins: [],
}
