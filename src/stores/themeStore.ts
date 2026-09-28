import { create } from 'zustand';
import { persist } from 'zustand/middleware';
import type { Theme } from '@mui/material/styles';
import { buildAppTheme } from '../theme';
import type { ThemeMode } from '../theme';

interface ThemeState {
  mode: ThemeMode;
  theme: Theme;
  setMode: (mode: ThemeMode) => void;
  toggleMode: () => void;
  initTheme: () => void;
}

const themes: Record<ThemeMode, Theme> = {
  light: buildAppTheme('light'),
  dark: buildAppTheme('dark'),
};

const syncDocumentClass = (mode: ThemeMode) => {
  document.documentElement.classList.toggle('dark', mode === 'dark');
};

export const useThemeStore = create<ThemeState>()(
  persist(
    (set, get) => ({
      mode: 'dark',
      theme: themes.dark,
      setMode: (mode) => {
        set({ mode, theme: themes[mode] });
        syncDocumentClass(mode);
      },
      toggleMode: () => {
        get().setMode(get().mode === 'light' ? 'dark' : 'light');
      },
      initTheme: () => {
        syncDocumentClass(get().mode);
      },
    }),
    {
      name: 'theme-storage',
      partialize: (state) => ({ mode: state.mode }),
    }
  )
);
