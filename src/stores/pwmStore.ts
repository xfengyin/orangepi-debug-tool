import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/tauri';
import type {
  PwmConfig,
  PwmChannelInfo,
  PwmWaveform,
  ApiResponse,
} from '../types';

interface PwmState {
  channels: PwmChannelInfo[];
  isLoading: boolean;
  error: string | null;
  
  setChannels: (channels: PwmChannelInfo[]) => void;
  setLoading: (loading: boolean) => void;
  setError: (error: string | null) => void;
  
  refreshChannels: () => Promise<void>;
  configureChannel: (config: PwmConfig) => Promise<boolean>;
  setFrequency: (chip: number, channel: number, frequency: number) => Promise<boolean>;
  setDutyCycle: (chip: number, channel: number, dutyCycle: number) => Promise<boolean>;
  setEnabled: (chip: number, channel: number, enabled: boolean) => Promise<boolean>;
  playWaveform: (chip: number, channel: number, waveform: PwmWaveform) => Promise<boolean>;
  unconfigureChannel: (chip: number, channel: number) => Promise<boolean>;
}

export const usePwmStore = create<PwmState>((set, get) => ({
  channels: [],
  isLoading: false,
  error: null,

  setChannels: (channels) => set({ channels }),
  setLoading: (isLoading) => set({ isLoading }),
  setError: (error) => set({ error }),

  refreshChannels: async () => {
    set({ isLoading: true, error: null });
    try {
      const response = await invoke<ApiResponse<PwmChannelInfo[]>>('list_pwm_channels');
      if (response.success && response.data) {
        set({ channels: response.data });
      } else {
        set({ error: response.error || 'Failed to list PWM channels' });
      }
    } catch (err) {
      set({ error: String(err) });
    } finally {
      set({ isLoading: false });
    }
  },

  configureChannel: async (config) => {
    try {
      const response = await invoke<ApiResponse<null>>('configure_pwm', { config });
      if (response.success) {
        await get().refreshChannels();
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

  setFrequency: async (chip, channel, frequency) => {
    try {
      const response = await invoke<ApiResponse<null>>('set_pwm_frequency', {
        chip,
        channel,
        frequency,
      });
      if (response.success) {
        await get().refreshChannels();
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

  setDutyCycle: async (chip, channel, dutyCycle) => {
    try {
      const response = await invoke<ApiResponse<null>>('set_pwm_duty_cycle', {
        chip,
        channel,
        dutyCycle,
      });
      if (response.success) {
        await get().refreshChannels();
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

  setEnabled: async (chip, channel, enabled) => {
    try {
      const response = await invoke<ApiResponse<null>>('set_pwm_enabled', {
        chip,
        channel,
        enabled,
      });
      if (response.success) {
        await get().refreshChannels();
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

  playWaveform: async (chip, channel, waveform) => {
    try {
      const response = await invoke<ApiResponse<null>>('play_pwm_waveform', {
        chip,
        channel,
        waveform,
      });
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

  unconfigureChannel: async (chip, channel) => {
    try {
      const response = await invoke<ApiResponse<null>>('unconfigure_pwm', {
        chip,
        channel,
      });
      if (response.success) {
        await get().refreshChannels();
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