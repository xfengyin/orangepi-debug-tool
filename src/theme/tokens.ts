// ===== 设计令牌：中性灰阶 + 单一品牌主色 + 语义色 =====
// 所有组件通过 useUiTokens() 或 MUI palette 语义引用取色，禁止散落硬编码色值。

export const sansFontFamily = '"Inter", system-ui, -apple-system, "Segoe UI", sans-serif';
export const monoFontFamily = '"JetBrains Mono", "SF Mono", Menlo, Consolas, monospace';

export type ThemeMode = 'light' | 'dark';

export interface SemanticColor {
  /** 主色（图标、文字、图表线） */
  main: string;
  /** 柔和底色（标签、徽章背景） */
  soft: string;
}

export interface ChartTokens {
  /** RX 数据线 */
  rx: string;
  /** TX 数据线 */
  tx: string;
  grid: string;
  axis: string;
  tooltipBg: string;
  tooltipBorder: string;
}

export interface UiTokens {
  mode: ThemeMode;
  /** 应用背景 */
  bg: string;
  /** 卡片/面板背景 */
  surface: string;
  /** 抬升层背景（表头、徽标、悬浮层） */
  surfaceAlt: string;
  /** 凹陷层背景（输入框、终端） */
  inset: string;
  border: string;
  borderStrong: string;
  text: string;
  textSecondary: string;
  textMuted: string;
  /** 品牌主色（OrangePi 橙），全站唯一强调色 */
  brand: string;
  brandHover: string;
  brandActive: string;
  brandSoft: string;
  success: SemanticColor;
  warning: SemanticColor;
  danger: SemanticColor;
  info: SemanticColor;
  chart: ChartTokens;
}

export const darkTokens: UiTokens = {
  mode: 'dark',
  bg: '#0b0c0e',
  surface: '#141518',
  surfaceAlt: '#1b1c21',
  inset: '#0f1013',
  border: 'rgba(255, 255, 255, 0.08)',
  borderStrong: 'rgba(255, 255, 255, 0.16)',
  text: '#ededf0',
  textSecondary: '#a2a6ad',
  textMuted: '#6e737c',
  brand: '#ff6b35',
  brandHover: '#ff8050',
  brandActive: '#e85a24',
  brandSoft: 'rgba(255, 107, 53, 0.14)',
  success: { main: '#3fc97f', soft: 'rgba(63, 201, 127, 0.12)' },
  warning: { main: '#e9a23b', soft: 'rgba(233, 162, 59, 0.12)' },
  danger: { main: '#f2555a', soft: 'rgba(242, 85, 90, 0.12)' },
  info: { main: '#5ea2f7', soft: 'rgba(94, 162, 247, 0.12)' },
  chart: {
    rx: '#3fc97f',
    tx: '#ff6b35',
    grid: 'rgba(255, 255, 255, 0.06)',
    axis: '#6e737c',
    tooltipBg: '#1b1c21',
    tooltipBorder: 'rgba(255, 255, 255, 0.12)',
  },
};

export const lightTokens: UiTokens = {
  mode: 'light',
  bg: '#f6f6f7',
  surface: '#ffffff',
  surfaceAlt: '#f0f0f2',
  inset: '#fbfbfc',
  border: 'rgba(0, 0, 0, 0.08)',
  borderStrong: 'rgba(0, 0, 0, 0.16)',
  text: '#1b1c1f',
  textSecondary: '#565b63',
  textMuted: '#878c95',
  brand: '#ff6b35',
  brandHover: '#f05a22',
  brandActive: '#d84e1b',
  brandSoft: 'rgba(255, 107, 53, 0.1)',
  success: { main: '#1f9d57', soft: 'rgba(31, 157, 87, 0.1)' },
  warning: { main: '#b7791f', soft: 'rgba(183, 121, 31, 0.1)' },
  danger: { main: '#d93a3e', soft: 'rgba(217, 58, 62, 0.08)' },
  info: { main: '#2b6fe3', soft: 'rgba(43, 111, 227, 0.08)' },
  chart: {
    rx: '#1f9d57',
    tx: '#ff6b35',
    grid: 'rgba(0, 0, 0, 0.06)',
    axis: '#878c95',
    tooltipBg: '#ffffff',
    tooltipBorder: 'rgba(0, 0, 0, 0.12)',
  },
};

export const tokensForMode = (mode: ThemeMode): UiTokens =>
  mode === 'light' ? lightTokens : darkTokens;
