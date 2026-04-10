import { create } from 'zustand';
import { persist } from 'zustand/middleware';
import { Theme } from '@mui/material/styles';
import { createTheme } from '@mui/material/styles';

interface ThemeState {
  mode: 'light' | 'dark';
  theme: Theme;
  setMode: (mode: 'light' | 'dark') => void;
  toggleMode: () => void;
  initTheme: () => void;
}

// ===== Cursor Dark Design System =====
// Primary: Dark bg (#0a0a0a) + Surface (#1a1a1a) + Gradient accent (purple→blue)

const cursorDarkTheme = createTheme({
  palette: {
    mode: 'dark',
    primary: {
      main: '#7c3aed',
      light: '#a78bfa',
      dark: '#5b21b6',
      contrastText: '#ffffff',
    },
    secondary: {
      main: '#3b82f6',
      light: '#60a5fa',
      dark: '#2563eb',
    },
    background: {
      default: '#0a0a0a',
      paper: '#1a1a1a',
    },
    error: {
      main: '#ef4444',
      light: '#f87171',
      dark: '#dc2626',
    },
    warning: {
      main: '#f59e0b',
      light: '#fbbf24',
      dark: '#d97706',
    },
    success: {
      main: '#4ade80',
      light: '#86efac',
      dark: '#22c55e',
    },
    info: {
      main: '#60a5fa',
      light: '#93c5fd',
      dark: '#3b82f6',
    },
    text: {
      primary: '#ffffff',
      secondary: '#a1a1aa',
    },
    divider: 'rgba(255, 255, 255, 0.08)',
  },
  typography: {
    fontFamily: '"Inter", "system-ui", "-apple-system", "Helvetica Neue", sans-serif',
    fontSize: 14,
    fontWeightLight: 300,
    fontWeightRegular: 400,
    fontWeightMedium: 500,
    fontWeightBold: 600,
    h5: {
      fontWeight: 600,
      letterSpacing: '-0.01em',
    },
    h6: {
      fontWeight: 600,
      letterSpacing: '-0.01em',
    },
    subtitle1: {
      fontWeight: 600,
    },
    subtitle2: {
      fontWeight: 500,
      color: '#a1a1aa',
    },
    caption: {
      color: '#71717a',
    },
    button: {
      textTransform: 'none',
      fontWeight: 500,
    },
  },
  shape: {
    borderRadius: 8,
  },
  components: {
    MuiCssBaseline: {
      styleOverrides: {
        body: {
          backgroundColor: '#0a0a0a',
          color: '#ffffff',
        },
      },
    },
    MuiButton: {
      styleOverrides: {
        root: {
          textTransform: 'none',
          borderRadius: 8,
          fontWeight: 500,
          padding: '8px 16px',
          fontSize: '0.875rem',
        },
        contained: {
          background: 'linear-gradient(135deg, #7c3aed, #3b82f6)',
          boxShadow: '0 0 20px rgba(124, 58, 237, 0.15)',
          '&:hover': {
            background: 'linear-gradient(135deg, #6d28d9, #2563eb)',
            boxShadow: '0 0 24px rgba(124, 58, 237, 0.25)',
          },
          '&:active': {
            background: 'linear-gradient(135deg, #5b21b6, #1d4ed8)',
          },
          '&:disabled': {
            background: '#2a2a2a',
            color: '#52525b',
            boxShadow: 'none',
          },
        },
        outlined: {
          borderColor: '#2a2a2a',
          color: '#a1a1aa',
          '&:hover': {
            borderColor: '#3a3a3a',
            backgroundColor: 'rgba(255, 255, 255, 0.03)',
          },
        },
        text: {
          color: '#a1a1aa',
          '&:hover': {
            backgroundColor: 'rgba(255, 255, 255, 0.05)',
            color: '#ffffff',
          },
        },
      },
    },
    MuiCard: {
      styleOverrides: {
        root: {
          borderRadius: 8,
          border: '1px solid #2a2a2a',
          backgroundColor: '#1a1a1a',
          backgroundImage: 'none',
          boxShadow: 'none',
        },
      },
    },
    MuiPaper: {
      styleOverrides: {
        root: {
          borderRadius: 8,
          backgroundImage: 'none',
          backgroundColor: '#1a1a1a',
        },
      },
    },
    MuiAppBar: {
      styleOverrides: {
        root: {
          boxShadow: 'none',
          backgroundImage: 'none',
          backgroundColor: '#0a0a0a',
          borderBottom: '1px solid #2a2a2a',
        },
      },
    },
    MuiDrawer: {
      styleOverrides: {
        paper: {
          borderRight: '1px solid #2a2a2a',
          backgroundImage: 'none',
          backgroundColor: '#0a0a0a',
        },
      },
    },
    MuiListItemButton: {
      styleOverrides: {
        root: {
          borderRadius: 6,
          margin: '2px 8px',
          padding: '8px 12px',
          color: '#a1a1aa',
          '&:hover': {
            backgroundColor: 'rgba(255, 255, 255, 0.05)',
            color: '#ffffff',
          },
          '&.Mui-selected': {
            backgroundColor: 'rgba(124, 58, 237, 0.12)',
            color: '#a78bfa',
            '&:hover': {
              backgroundColor: 'rgba(124, 58, 237, 0.18)',
            },
          },
        },
      },
    },
    MuiListItemIcon: {
      styleOverrides: {
        root: {
          minWidth: 36,
          color: 'inherit',
        },
      },
    },
    MuiChip: {
      styleOverrides: {
        root: {
          borderRadius: 6,
          fontWeight: 500,
          fontSize: '0.75rem',
        },
        outlined: {
          borderColor: '#2a2a2a',
          color: '#a1a1aa',
        },
        filled: {
          backgroundColor: '#1a1a1a',
        },
      },
    },
    MuiTextField: {
      styleOverrides: {
        root: {
          '& .MuiOutlinedInput-root': {
            borderRadius: 8,
            backgroundColor: '#0a0a0a',
            '& fieldset': {
              borderColor: '#2a2a2a',
            },
            '&:hover fieldset': {
              borderColor: '#3a3a3a',
            },
            '&.Mui-focused fieldset': {
              borderColor: '#7c3aed',
              boxShadow: '0 0 0 2px rgba(124, 58, 237, 0.2)',
            },
          },
          '& .MuiInputLabel-root': {
            color: '#71717a',
            '&.Mui-focused': {
              color: '#a78bfa',
            },
          },
          '& .MuiInputBase-input': {
            color: '#ffffff',
            fontFamily: '"Inter", sans-serif',
          },
        },
      },
    },
    MuiSelect: {
      styleOverrides: {
        root: {
          borderRadius: 8,
        },
      },
    },
    MuiSwitch: {
      styleOverrides: {
        root: {
          '& .MuiSwitch-switchBase.Mui-checked': {
            color: '#7c3aed',
          },
          '& .MuiSwitch-switchBase.Mui-checked + .MuiSwitch-track': {
            backgroundColor: '#7c3aed',
          },
          '& .MuiSwitch-track': {
            backgroundColor: '#2a2a2a',
          },
        },
      },
    },
    MuiSlider: {
      styleOverrides: {
        root: {
          color: '#7c3aed',
          '& .MuiSlider-track': {
            background: 'linear-gradient(90deg, #7c3aed, #3b82f6)',
          },
          '& .MuiSlider-thumb': {
            boxShadow: '0 0 8px rgba(124, 58, 237, 0.4)',
          },
        },
      },
    },
    MuiTable: {
      styleOverrides: {
        root: {
          '& .MuiTableCell-head': {
            backgroundColor: '#141414',
            color: '#a1a1aa',
            fontWeight: 600,
            borderBottom: '1px solid #2a2a2a',
          },
          '& .MuiTableCell-body': {
            borderBottom: '1px solid rgba(255, 255, 255, 0.04)',
            color: '#e4e4e7',
          },
          '& .MuiTableRow-root:hover': {
            backgroundColor: 'rgba(255, 255, 255, 0.02)',
          },
        },
      },
    },
    MuiDivider: {
      styleOverrides: {
        root: {
          borderColor: '#2a2a2a',
        },
      },
    },
    MuiTooltip: {
      styleOverrides: {
        tooltip: {
          backgroundColor: '#2a2a2a',
          color: '#e4e4e7',
          borderRadius: 6,
          fontSize: '0.75rem',
          border: '1px solid #3a3a3a',
        },
      },
    },
    MuiSnackbar: {
      styleOverrides: {
        root: {
          '& .MuiAlert-root': {
            borderRadius: 8,
            border: '1px solid #2a2a2a',
          },
        },
      },
    },
    MuiAlert: {
      styleOverrides: {
        root: {
          backgroundColor: '#1a1a1a',
          color: '#e4e4e7',
        },
        standardError: {
          borderColor: 'rgba(239, 68, 68, 0.3)',
        },
        standardSuccess: {
          borderColor: 'rgba(74, 222, 128, 0.3)',
        },
        standardWarning: {
          borderColor: 'rgba(245, 158, 11, 0.3)',
        },
        standardInfo: {
          borderColor: 'rgba(96, 165, 250, 0.3)',
        },
      },
    },
    MuiToggleButton: {
      styleOverrides: {
        root: {
          color: '#71717a',
          border: '1px solid #2a2a2a',
          '&.Mui-selected': {
            backgroundColor: 'rgba(124, 58, 237, 0.15)',
            color: '#a78bfa',
            borderColor: '#7c3aed',
          },
          '&:hover': {
            backgroundColor: 'rgba(255, 255, 255, 0.05)',
          },
        },
      },
    },
    MuiIconButton: {
      styleOverrides: {
        root: {
          color: '#71717a',
          '&:hover': {
            color: '#e4e4e7',
            backgroundColor: 'rgba(255, 255, 255, 0.05)',
          },
        },
      },
    },
  },
});

// Light theme (simplified - mirrors dark structure with light tones)
const cursorLightTheme = createTheme({
  palette: {
    mode: 'light',
    primary: {
      main: '#7c3aed',
      light: '#a78bfa',
      dark: '#5b21b6',
      contrastText: '#ffffff',
    },
    secondary: {
      main: '#3b82f6',
      light: '#60a5fa',
      dark: '#2563eb',
    },
    background: {
      default: '#fafafa',
      paper: '#ffffff',
    },
    error: {
      main: '#ef4444',
      light: '#f87171',
      dark: '#dc2626',
    },
    warning: {
      main: '#f59e0b',
      light: '#fbbf24',
      dark: '#d97706',
    },
    success: {
      main: '#22c55e',
      light: '#86efac',
      dark: '#16a34a',
    },
    info: {
      main: '#3b82f6',
      light: '#93c5fd',
      dark: '#2563eb',
    },
    text: {
      primary: '#18181b',
      secondary: '#71717a',
    },
    divider: 'rgba(0, 0, 0, 0.08)',
  },
  typography: {
    fontFamily: '"Inter", "system-ui", "-apple-system", "Helvetica Neue", sans-serif',
    fontSize: 14,
    fontWeightLight: 300,
    fontWeightRegular: 400,
    fontWeightMedium: 500,
    fontWeightBold: 600,
  },
  shape: {
    borderRadius: 8,
  },
  components: {
    MuiButton: {
      styleOverrides: {
        root: {
          textTransform: 'none',
          borderRadius: 8,
          fontWeight: 500,
        },
        contained: {
          background: 'linear-gradient(135deg, #7c3aed, #3b82f6)',
          '&:hover': {
            background: 'linear-gradient(135deg, #6d28d9, #2563eb)',
          },
        },
      },
    },
    MuiCard: {
      styleOverrides: {
        root: {
          borderRadius: 8,
          border: '1px solid rgba(0,0,0,0.08)',
        },
      },
    },
    MuiListItemButton: {
      styleOverrides: {
        root: {
          borderRadius: 6,
          margin: '2px 8px',
          '&.Mui-selected': {
            backgroundColor: 'rgba(124, 58, 237, 0.08)',
            '&:hover': {
              backgroundColor: 'rgba(124, 58, 237, 0.12)',
            },
          },
        },
      },
    },
    MuiTextField: {
      styleOverrides: {
        root: {
          '& .MuiOutlinedInput-root.Mui-focused fieldset': {
            borderColor: '#7c3aed',
          },
          '& .MuiInputLabel-root.Mui-focused': {
            color: '#7c3aed',
          },
        },
      },
    },
  },
});

export const useThemeStore = create<ThemeState>()(
  persist(
    (set, get) => ({
      mode: 'dark',
      theme: cursorDarkTheme,
      setMode: (mode) => {
        set({ mode, theme: mode === 'light' ? cursorLightTheme : cursorDarkTheme });
        // Update document class for Tailwind
        if (mode === 'dark') {
          document.documentElement.classList.add('dark');
        } else {
          document.documentElement.classList.remove('dark');
        }
      },
      toggleMode: () => {
        const newMode = get().mode === 'light' ? 'dark' : 'light';
        get().setMode(newMode);
      },
      initTheme: () => {
        const savedMode = get().mode;
        if (savedMode === 'dark') {
          document.documentElement.classList.add('dark');
        } else {
          document.documentElement.classList.remove('dark');
        }
      },
    }),
    {
      name: 'theme-storage',
      partialize: (state) => ({ mode: state.mode }),
    }
  )
);
