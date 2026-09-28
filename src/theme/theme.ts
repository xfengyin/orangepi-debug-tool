import { createTheme } from '@mui/material/styles';
import type { Shadows, Theme } from '@mui/material/styles';
import { monoFontFamily, sansFontFamily, tokensForMode } from './tokens';
import type { ThemeMode, UiTokens } from './tokens';

/** 阴影层级：0 无 / 1 卡片静止 / 2 悬浮 hover / 3 弹层，其余级别取最近值 */
const buildShadows = (mode: ThemeMode): Shadows => {
  const umb = mode === 'dark' ? 'rgba(0,0,0' : 'rgba(15,17,21';
  const level1 = `0 1px 2px ${umb},0.16)`;
  const level2 = `0 4px 12px ${umb},0.24)`;
  const level3 = `0 12px 32px ${umb},0.32)`;
  const shadows = Array<string>(25).fill(level3);
  shadows[0] = 'none';
  shadows[1] = level1;
  shadows[2] = level2;
  return shadows as unknown as Shadows;
};

const componentOverrides = (t: UiTokens): Theme['components'] => ({
  MuiCssBaseline: {
    styleOverrides: {
      body: {
        backgroundColor: t.bg,
        color: t.text,
      },
    },
  },
  MuiButton: {
    defaultProps: { disableElevation: true },
    styleOverrides: {
      root: {
        textTransform: 'none',
        borderRadius: 8,
        fontWeight: 500,
        padding: '7px 14px',
        fontSize: '0.8125rem',
        boxShadow: 'none',
      },
      contained: {
        backgroundColor: t.brand,
        color: '#ffffff',
        '&:hover': { backgroundColor: t.brandHover, boxShadow: 'none' },
        '&:active': { backgroundColor: t.brandActive },
        '&.Mui-disabled': {
          backgroundColor: t.surfaceAlt,
          color: t.textMuted,
        },
      },
      outlined: {
        borderColor: t.border,
        color: t.textSecondary,
        '&:hover': {
          borderColor: t.borderStrong,
          backgroundColor: t.surfaceAlt,
          color: t.text,
        },
      },
      text: {
        color: t.textSecondary,
        '&:hover': { backgroundColor: t.surfaceAlt, color: t.text },
      },
    },
  },
  MuiCard: {
    defaultProps: { elevation: 1 },
    styleOverrides: {
      root: {
        borderRadius: 10,
        border: `1px solid ${t.border}`,
        backgroundColor: t.surface,
        backgroundImage: 'none',
      },
    },
  },
  MuiPaper: {
    styleOverrides: {
      root: {
        backgroundImage: 'none',
        backgroundColor: t.surface,
      },
    },
  },
  MuiAppBar: {
    styleOverrides: {
      root: {
        boxShadow: 'none',
        backgroundImage: 'none',
        backgroundColor: t.bg,
        borderBottom: `1px solid ${t.border}`,
      },
    },
  },
  MuiDrawer: {
    styleOverrides: {
      paper: {
        borderRight: `1px solid ${t.border}`,
        backgroundImage: 'none',
        backgroundColor: t.bg,
      },
    },
  },
  MuiListItemButton: {
    styleOverrides: {
      root: {
        borderRadius: 8,
        margin: '1px 8px',
        padding: '7px 12px',
        color: t.textMuted,
        '&:hover': {
          backgroundColor: t.surfaceAlt,
          color: t.text,
        },
        '&.Mui-selected': {
          backgroundColor: t.brandSoft,
          color: t.brand,
          '&:hover': { backgroundColor: t.brandSoft },
        },
      },
    },
  },
  MuiListItemIcon: {
    styleOverrides: {
      root: { minWidth: 34, color: 'inherit' },
    },
  },
  MuiChip: {
    styleOverrides: {
      root: {
        borderRadius: 6,
        fontWeight: 500,
        fontSize: '0.75rem',
      },
      outlined: { borderColor: t.border, color: t.textSecondary },
      filled: { backgroundColor: t.surfaceAlt },
    },
  },
  MuiTextField: {
    styleOverrides: {
      root: {
        '& .MuiOutlinedInput-root': {
          borderRadius: 8,
          backgroundColor: t.inset,
          '& fieldset': { borderColor: t.border },
          '&:hover fieldset': { borderColor: t.borderStrong },
          '&.Mui-focused fieldset': {
            borderColor: t.brand,
            borderWidth: 1,
          },
        },
        '& .MuiInputLabel-root': {
          color: t.textMuted,
          '&.Mui-focused': { color: t.brand },
        },
        '& .MuiInputBase-input': {
          color: t.text,
          fontFamily: sansFontFamily,
        },
      },
    },
  },
  MuiSelect: {
    styleOverrides: {
      root: { borderRadius: 8 },
      icon: { color: t.textMuted },
    },
  },
  MuiSwitch: {
    styleOverrides: {
      root: {
        '& .MuiSwitch-switchBase.Mui-checked': { color: t.brand },
        '& .MuiSwitch-switchBase.Mui-checked + .MuiSwitch-track': {
          backgroundColor: t.brand,
          opacity: 0.55,
        },
        '& .MuiSwitch-track': {
          backgroundColor: t.mode === 'dark' ? 'rgba(255,255,255,0.16)' : 'rgba(0,0,0,0.16)',
        },
      },
    },
  },
  MuiSlider: {
    styleOverrides: {
      root: {
        color: t.brand,
        '& .MuiSlider-rail': { color: t.surfaceAlt, opacity: 1 },
        '& .MuiSlider-thumb': {
          boxShadow: 'none',
          '&:hover, &.Mui-focusVisible': {
            boxShadow: `0 0 0 6px ${t.brandSoft}`,
          },
        },
        '& .MuiSlider-markLabel': {
          color: t.textMuted,
          fontSize: '0.65rem',
          fontFamily: monoFontFamily,
        },
      },
    },
  },
  MuiTable: {
    styleOverrides: {
      root: {
        '& .MuiTableCell-head': {
          backgroundColor: t.surfaceAlt,
          color: t.textMuted,
          fontWeight: 600,
          borderBottom: `1px solid ${t.border}`,
        },
        '& .MuiTableCell-body': {
          borderBottom: `1px solid ${t.border}`,
          color: t.text,
        },
        '& .MuiTableRow-root:hover': {
          backgroundColor: t.mode === 'dark' ? 'rgba(255,255,255,0.02)' : 'rgba(0,0,0,0.02)',
        },
      },
    },
  },
  MuiDivider: {
    styleOverrides: {
      root: { borderColor: t.border },
    },
  },
  MuiTooltip: {
    styleOverrides: {
      tooltip: {
        backgroundColor: t.surfaceAlt,
        color: t.text,
        borderRadius: 6,
        fontSize: '0.75rem',
        border: `1px solid ${t.border}`,
        boxShadow: 'none',
      },
      arrow: { color: t.surfaceAlt },
    },
  },
  MuiAlert: {
    styleOverrides: {
      root: {
        backgroundColor: t.surface,
        color: t.text,
        borderRadius: 8,
        border: `1px solid ${t.border}`,
      },
      standardError: { borderColor: t.danger.soft, color: t.danger.main },
      standardSuccess: { borderColor: t.success.soft },
      standardWarning: { borderColor: t.warning.soft },
      standardInfo: { borderColor: t.info.soft },
    },
  },
  MuiToggleButton: {
    styleOverrides: {
      root: {
        color: t.textMuted,
        border: `1px solid ${t.border}`,
        textTransform: 'none',
        '&.Mui-selected': {
          backgroundColor: t.brandSoft,
          color: t.brand,
          borderColor: t.brand,
          '&:hover': { backgroundColor: t.brandSoft },
        },
        '&:hover': { backgroundColor: t.surfaceAlt },
      },
    },
  },
  MuiIconButton: {
    styleOverrides: {
      root: {
        color: t.textMuted,
        borderRadius: 8,
        '&:hover': {
          color: t.text,
          backgroundColor: t.surfaceAlt,
        },
      },
    },
  },
  MuiDialog: {
    styleOverrides: {
      paper: {
        backgroundColor: t.surface,
        backgroundImage: 'none',
        border: `1px solid ${t.border}`,
        borderRadius: 12,
      },
    },
  },
});

/** 按模式构建应用主题（Linear/Vercel 式：中性灰 + 单一品牌橙，无渐变） */
export const buildAppTheme = (mode: ThemeMode): Theme => {
  const t = tokensForMode(mode);
  return createTheme({
    palette: {
      mode,
      primary: {
        main: t.brand,
        light: t.brandHover,
        dark: t.brandActive,
        contrastText: '#ffffff',
      },
      secondary: {
        main: t.textSecondary,
        light: t.text,
        dark: t.textMuted,
        contrastText: t.text,
      },
      background: {
        default: t.bg,
        paper: t.surface,
      },
      error: { main: t.danger.main },
      warning: { main: t.warning.main },
      success: { main: t.success.main },
      info: { main: t.info.main },
      text: {
        primary: t.text,
        secondary: t.textSecondary,
        disabled: t.textMuted,
      },
      divider: t.border,
      action: {
        hover: t.mode === 'dark' ? 'rgba(255,255,255,0.04)' : 'rgba(0,0,0,0.04)',
        selected: t.brandSoft,
      },
    },
    typography: {
      fontFamily: sansFontFamily,
      fontSize: 13,
      fontWeightLight: 300,
      fontWeightRegular: 400,
      fontWeightMedium: 500,
      fontWeightBold: 600,
      h5: { fontSize: '1.125rem', fontWeight: 600, letterSpacing: '-0.01em' },
      h6: { fontSize: '0.9375rem', fontWeight: 600, letterSpacing: '-0.01em' },
      subtitle1: { fontSize: '0.875rem', fontWeight: 600 },
      subtitle2: { fontSize: '0.8125rem', fontWeight: 500, color: t.textSecondary },
      body1: { fontSize: '0.875rem' },
      body2: { fontSize: '0.8125rem' },
      caption: { fontSize: '0.75rem', color: t.textMuted },
      button: { textTransform: 'none', fontWeight: 500 },
      overline: {
        fontSize: '0.6875rem',
        fontWeight: 600,
        letterSpacing: '0.06em',
        fontFamily: monoFontFamily,
        textTransform: 'uppercase',
      },
    },
    shape: { borderRadius: 10 },
    shadows: buildShadows(mode),
    components: componentOverrides(t),
  });
};
