import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/tauri';
import type {
  GpioConfig,
  GpioPinInfo,
  GpioPinState,
  ApiResponse,
} from '../types';

interface GpioState {
  pins: GpioPinInfo[];
  pinStates: Map<number, GpioPinState>;
  isLoading: boolean;
  error: string | null;
  
  setPins: (pins: GpioPinInfo[]) => void;
  setPinState: (pin: number, state: GpioPinState) => void;
  setLoading: (loading: boolean) => void;
  setError: (error: string | null) => void;
  
  refreshPins: () => Promise<void>;
  configurePin: (config: GpioConfig) => Promise<boolean>;
  readPin: (pin: number) => Promise<number | null>;
  writePin: (pin: number, value: number) => Promise<boolean>;
  togglePin: (pin: number) => Promise<number | null>;
  unconfigurePin: (pin: number) => Promise<boolean>;
}

export const useGpioStore = create<GpioState>((set, get) => ({
  pins: [],
  pinStates: new Map(),
  isLoading: false,
  error: null,

  setPins: (pins) => set({ pins }),
  setPinState: (pin, state) => set((s) => ({
    pinStates: new Map(s.pinStates).set(pin, state),
  })),
  setLoading: (isLoading) => set({ isLoading }),
  setError: (error) => set({ error }),

  refreshPins: async () => {
    set({ isLoading: true, error: null });
    try {
      const response = await invoke<ApiResponse<GpioPinInfo[]>>('list_gpio_pins');
      if (response.success && response.data) {
        set({ pins: response.data });
      } else {
        set({ error: response.error || 'Failed to list GPIO pins' });
      }
    } catch (err) {
      set({ error: String(err) });
    } finally {
      set({ isLoading: false });
    }
  },

  configurePin: async (config) => {
    try {
      const response = await invoke<ApiResponse<null>>('configure_gpio', { config });
      if (response.success) {
        return true;
      } else {
        set({ error: response.error || 'Failed to configure pin' });
        return false;
      }
    } catch (err) {
      set({ error: String(err) });
      return false;
    }
  },

  readPin: async (pin) => {
    try {
      const response = await invoke<ApiResponse<number>>('read_gpio', { pin });
      if (response.success) {
        return response.data ?? null;
      } else {
        set({ error: response.error });
        return null;
      }
    } catch (err) {
      set({ error: String(err) });
      return null;
    }
  },

  writePin: async (pin, value) => {
    try {
      const response = await invoke<ApiResponse<null>>('write_gpio', { pin, value });
      if (response.success) {
        return true;
      } else {
        set({ error: response.error });
        return false;
      }
    } catch (err) {
      set({ error: String(err) });
      return false;
    }
  },

  togglePin: async (pin) => {
    try {
      const response = await invoke<ApiResponse<number>>('toggle_gpio', { pin });
      if (response.success) {
        return response.data ?? null;
      } else {
        set({ error: response.error });
        return null;
      }
    } catch (err) {
      set({ error: String(err) });
      return null;
    }
  },

  unconfigurePin: async (pin) => {
    try {
      const response = await invoke<ApiResponse<null>>('unconfigure_gpio', { pin });
      if (response.success) {
        return true;
      } else {
        set({ error: response.error });
        return false;
      }
    } catch (err) {
      set({ error: String(err) });
      return false;
    }
  },
}));