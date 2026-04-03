import { create } from 'zustand';
import { persist } from 'zustand/middleware';
import { invoke } from '@tauri-apps/api/tauri';
import type { AppConfig, SystemInfo, ViewType, ApiResponse, ToastMessage } from '../types';
import { v4 as uuidv4 } from 'uuid';

interface AppState {
  // Navigation
  currentView: ViewType;
  setCurrentView: (view: ViewType) => void;
  
  // Configuration
  config: AppConfig;
  updateConfig: (config: Partial<AppConfig>) => void;
  loadConfig: () => Promise<void>;
  saveConfig: () => Promise<void>;
  
  // System info
  systemInfo: SystemInfo | null;
  isOrangePi: boolean;
  loadSystemInfo: () => Promise<void>;
  checkOrangePi: () => Promise<void>;
  
  // UI state
  isSidebarOpen: boolean;
  toggleSidebar: () => void;
  
  // Toast notifications
  toasts: ToastMessage[];
  addToast: (message: string, type?: ToastMessage['type'], duration?: number) => void;
  removeToast: (id: string) => void;
  
  // Loading states
  isLoading: boolean;
  setLoading: (loading: boolean) => void;
  
  // Error handling
  globalError: string | null;
  setGlobalError: (error: string | null) => void;
}

const defaultConfig: AppConfig = {
  theme: 'system',
  language: 'zh-CN',
  auto_save_interval: 30,
  serial_buffer_size: 65536,
  max_log_entries: 10000,
  hardware_acceleration: true,
};

export const useAppStore = create<AppState>()(
  persist(
    (set, get) => ({
      currentView: 'serial',
      config: defaultConfig,
      systemInfo: null,
      isOrangePi: false,
      isSidebarOpen: true,
      toasts: [],
      isLoading: false,
      globalError: null,

      setCurrentView: (view) => set({ currentView: view }),

      updateConfig: (newConfig) => set((state) => ({
        config: { ...state.config, ...newConfig },
      })),

      loadConfig: async () => {
        try {
          const response = await invoke<ApiResponse<AppConfig>>('get_config');
          if (response.success && response.data) {
            set({ config: { ...defaultConfig, ...response.data } });
          }
        } catch (err) {
          console.error('Failed to load config:', err);
        }
      },

      saveConfig: async () => {
        const { config } = get();
        try {
          await invoke<ApiResponse<null>>('update_config', { config });
        } catch (err) {
          console.error('Failed to save config:', err);
        }
      },

      loadSystemInfo: async () => {
        try {
          const response = await invoke<ApiResponse<SystemInfo>>('get_system_info');
          if (response.success && response.data) {
            set({ systemInfo: response.data });
          }
        } catch (err) {
          console.error('Failed to load system info:', err);
        }
      },

      checkOrangePi: async () => {
        try {
          const response = await invoke<ApiResponse<boolean>>('check_orangepi');
          if (response.success) {
            set({ isOrangePi: response.data ?? false });
          }
        } catch (err) {
          console.error('Failed to check OrangePi:', err);
        }
      },

      toggleSidebar: () => set((state) => ({ isSidebarOpen: !state.isSidebarOpen })),

      addToast: (message, type = 'info', duration = 3000) => {
        const toast: ToastMessage = {
          id: uuidv4(),
          type,
          message,
          duration,
        };
        set((state) => ({ toasts: [...state.toasts, toast] }));
        
        // Auto remove
        setTimeout(() => {
          get().removeToast(toast.id);
        }, duration);
      },

      removeToast: (id) => set((state) => ({
        toasts: state.toasts.filter((t) => t.id !== id),
      })),

      setLoading: (isLoading) => set({ isLoading }),

      setGlobalError: (globalError) => set({ globalError }),
    }),
    {
      name: 'app-storage',
      partialize: (state) => ({
        config: state.config,
        isSidebarOpen: state.isSidebarOpen,
      }),
    }
  )
);