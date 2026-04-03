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

// Brand color
const PRIMARY_COLOR = '#165DFF';

// Light theme
const lightTheme = createTheme({
  palette: {
    mode: 'light',
    primary: {
      main: PRIMARY_COLOR,
      light: '#4d85ff',
      dark: '#0041c4',
      contrastText: '#fff',
    },
    secondary: {
      main: '#64748b',
      light: '#94a3b8',
      dark: '#475569',
    },
    background: {
      default: '#f8fafc',
      paper: '#ffffff',
    },
    error: {
      main: '#ef4444',
      light: '#fca5a5',
      dark: '#b91c1c',
    },
    warning: {
      main: '#f59e0b',
      light: '#fcd34d',
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
      primary: '#1e293b',
      secondary: '#64748b',
    },
    divider: 'rgba(0, 0, 0, 0.12)',
  },
  typography: {
    fontFamily: '"Roboto", "Helvetica", "Arial", sans-serif',
    fontSize: 14,
    fontWeightLight: 300,
    fontWeightRegular: 400,
    fontWeightMedium: 500,
    fontWeightBold: 700,
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
      },
    },
    MuiCard: {
      styleOverrides: {
        root: {
          borderRadius: 12,
          boxShadow: '0 1px 3px rgba(0,0,0,0.12), 0 1px 2px rgba(0,0,0,0.24)',
        },
      },
    },
    MuiPaper: {
      styleOverrides: {
        root: {
          borderRadius: 12,
        },
      },
    },
    MuiAppBar: {
      styleOverrides: {
        root: {
          boxShadow: '0 1px 3px rgba(0,0,0,0.12)',
        },
      },
    },
    MuiDrawer: {
      styleOverrides: {
        paper: {
          borderRight: 'none',
        },
      },
    },
    MuiListItemButton: {
      styleOverrides: {
        root: {
          borderRadius: 8,
          margin: '4px 8px',
          '&.Mui-selected': {
            backgroundColor: 'rgba(22, 93, 255, 0.12)',
            '&:hover': {
              backgroundColor: 'rgba(22, 93, 255, 0.18)',
            },
          },
        },
      },
    },
  },
});

// Dark theme
const darkTheme = createTheme({
  palette: {
    mode: 'dark',
    primary: {
      main: '#4d85ff',
      light: '#7aa8ff',
      dark: '#165DFF',
      contrastText: '#fff',
    },
    secondary: {
      main: '#94a3b8',
      light: '#cbd5e1',
      dark: '#64748b',
    },
    background: {
      default: '#0f172a',
      paper: '#1e293b',
    },
    error: {
      main: '#ef4444',
      light: '#fca5a5',
      dark: '#b91c1c',
    },
    warning: {
      main: '#f59e0b',
      light: '#fcd34d',
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
      primary: '#f1f5f9',
      secondary: '#94a3b8',
    },
    divider: 'rgba(255, 255, 255, 0.12)',
  },
  typography: {
    fontFamily: '"Roboto", "Helvetica", "Arial", sans-serif',
    fontSize: 14,
    fontWeightLight: 300,
    fontWeightRegular: 400,
    fontWeightMedium: 500,
    fontWeightBold: 700,
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
      },
    },
    MuiCard: {
      styleOverrides: {
        root: {
          borderRadius: 12,
          boxShadow: '0 1px 3px rgba(0,0,0,0.24), 0 1px 2px rgba(0,0,0,0.36)',
          backgroundImage: 'none',
        },
      },
    },
    MuiPaper: {
      styleOverrides: {
        root: {
          borderRadius: 12,
          backgroundImage: 'none',
        },
      },
    },
    MuiAppBar: {
      styleOverrides: {
        root: {
          boxShadow: '0 1px 3px rgba(0,0,0,0.24)',
          backgroundImage: 'none',
        },
      },
    },
    MuiDrawer: {
      styleOverrides: {
        paper: {
          borderRight: 'none',
          backgroundImage: 'none',
        },
      },
    },
    MuiListItemButton: {
      styleOverrides: {
        root: {
          borderRadius: 8,
          margin: '4px 8px',
          '&.Mui-selected': {
            backgroundColor: 'rgba(77, 133, 255, 0.16)',
            '&:hover': {
              backgroundColor: 'rgba(77, 133, 255, 0.24)',
            },
          },
        },
      },
    },
  },
});

export const useThemeStore = create<ThemeState>()(
  persist(
    (set, get) => ({
      mode: 'light',
      theme: lightTheme,
      setMode: (mode) => {
        set({ mode, theme: mode === 'light' ? lightTheme : darkTheme });
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
        }
      },
    }),
    {
      name: 'theme-storage',
      partialize: (state) => ({ mode: state.mode }),
    }
  )
);