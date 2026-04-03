import { create } from 'zustand';
import { persist } from 'zustand/middleware';
import { invoke } from '@tauri-apps/api/tauri';
import type {
  SerialPortInfo,
  SerialConfig,
  SerialStatus,
  SerialDataPacket,
  ApiResponse,
} from '../types';

interface SerialState {
  // State
  ports: SerialPortInfo[];
  selectedPort: string | null;
  config: SerialConfig;
  status: SerialStatus;
  dataBuffer: SerialDataPacket[];
  isConnecting: boolean;
  error: string | null;
  
  // Actions
  setPorts: (ports: SerialPortInfo[]) => void;
  setSelectedPort: (port: string | null) => void;
  setConfig: (config: Partial<SerialConfig>) => void;
  setStatus: (status: Partial<SerialStatus>) => void;
  addData: (data: SerialDataPacket) => void;
  clearData: () => void;
  setConnecting: (connecting: boolean) => void;
  setError: (error: string | null) => void;
  
  // Async actions
  refreshPorts: () => Promise<void>;
  autoDetect: () => Promise<string | null>;
  connect: () => Promise<boolean>;
  disconnect: () => Promise<boolean>;
  write: (data: string | number[]) => Promise<boolean>;
  writeLine: (data: string) => Promise<boolean>;
}

const defaultConfig: SerialConfig = {
  port_name: '',
  baud_rate: 115200,
  data_bits: 8,
  parity: 'none',
  stop_bits: 1,
  flow_control: 'none',
  read_timeout: 1000,
  write_timeout: 1000,
};

export const useSerialStore = create<SerialState>()(
  persist(
    (set, get) => ({
      ports: [],
      selectedPort: null,
      config: defaultConfig,
      status: {
        connected: false,
        rx_bytes: 0,
        tx_bytes: 0,
        is_monitoring: false,
      },
      dataBuffer: [],
      isConnecting: false,
      error: null,

      setPorts: (ports) => set({ ports }),
      setSelectedPort: (port) => set({ selectedPort: port }),
      setConfig: (config) => set((state) => ({
        config: { ...state.config, ...config },
      })),
      setStatus: (status) => set((state) => ({
        status: { ...state.status, ...status },
      })),
      addData: (data) => set((state) => ({
        dataBuffer: [...state.dataBuffer.slice(-9999), data],
        status: {
          ...state.status,
          rx_bytes: state.status.rx_bytes + (data.is_rx ? data.data.length : 0),
          tx_bytes: state.status.tx_bytes + (!data.is_rx ? data.data.length : 0),
        },
      })),
      clearData: () => set({ dataBuffer: [] }),
      setConnecting: (isConnecting) => set({ isConnecting }),
      setError: (error) => set({ error }),

      refreshPorts: async () => {
        try {
          const response = await invoke<ApiResponse<SerialPortInfo[]>>('list_serial_ports');
          if (response.success && response.data) {
            set({ ports: response.data });
          } else {
            set({ error: response.error || 'Failed to list ports' });
          }
        } catch (err) {
          set({ error: String(err) });
        }
      },

      autoDetect: async () => {
        try {
          const response = await invoke<ApiResponse<string | null>>('auto_detect_serial');
          if (response.success) {
            if (response.data) {
              set((state) => ({
                selectedPort: response.data,
                config: { ...state.config, port_name: response.data! },
              }));
            }
            return response.data;
          }
          return null;
        } catch (err) {
          set({ error: String(err) });
          return null;
        }
      },

      connect: async () => {
        const { config, setConnecting, setStatus, setError } = get();
        
        if (!config.port_name) {
          setError('Please select a port');
          return false;
        }

        setConnecting(true);
        setError(null);

        try {
          const response = await invoke<ApiResponse<null>>('connect_serial', { config });
          if (response.success) {
            setStatus({ connected: true });
            return true;
          } else {
            setError(response.error || 'Failed to connect');
            return false;
          }
        } catch (err) {
          setError(String(err));
          return false;
        } finally {
          setConnecting(false);
        }
      },

      disconnect: async () => {
        const { setStatus, setError } = get();
        
        try {
          const response = await invoke<ApiResponse<null>>('disconnect_serial');
          if (response.success) {
            setStatus({ connected: false });
            return true;
          } else {
            setError(response.error || 'Failed to disconnect');
            return false;
          }
        } catch (err) {
          setError(String(err));
          return false;
        }
      },

      write: async (data) => {
        const { status } = get();
        
        if (!status.connected) {
          set({ error: 'Not connected' });
          return false;
        }

        try {
          let response;
          if (typeof data === 'string') {
            response = await invoke<ApiResponse<number>>('write_serial_string', { data });
          } else {
            response = await invoke<ApiResponse<number>>('write_serial', { data });
          }
          
          if (response.success) {
            return true;
          } else {
            set({ error: response.error || 'Failed to write' });
            return false;
          }
        } catch (err) {
          set({ error: String(err) });
          return false;
        }
      },

      writeLine: async (data) => {
        const line = data.endsWith('\n') ? data : `${data}\n`;
        return get().write(line);
      },
    }),
    {
      name: 'serial-storage',
      partialize: (state) => ({
        config: state.config,
        selectedPort: state.selectedPort,
      }),
    }
  )
);