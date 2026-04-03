// Application types

// ==================== Serial Types ====================

export interface SerialPortInfo {
  port_name: string;
  port_type: string;
  vid?: number;
  pid?: number;
  serial_number?: string;
  manufacturer?: string;
  product?: string;
}

export interface SerialConfig {
  port_name: string;
  baud_rate: number;
  data_bits: number;
  parity: 'none' | 'even' | 'odd';
  stop_bits: number;
  flow_control: 'none' | 'software' | 'hardware';
  read_timeout: number;
  write_timeout: number;
}

export interface SerialStatus {
  connected: boolean;
  config?: SerialConfig;
  rx_bytes: number;
  tx_bytes: number;
  is_monitoring: boolean;
}

export interface SerialDataPacket {
  timestamp: number;
  data: number[];
  is_rx: boolean;
}

// ==================== GPIO Types ====================

export interface GpioConfig {
  pin: number;
  direction: 'input' | 'output';
  pull: 'none' | 'up' | 'down';
  initial_value: number;
  interrupt?: GpioInterruptConfig;
}

export interface GpioInterruptConfig {
  trigger: 'rising' | 'falling' | 'both' | 'high' | 'low';
  debounce_ms: number;
}

export interface GpioPinInfo {
  pin: number;
  name: string;
  modes: string[];
  current_mode?: string;
  is_exported: boolean;
}

export interface GpioPinState {
  pin: number;
  direction: string;
  value: number;
  pull: string;
  interrupt_enabled: boolean;
  interrupt_trigger?: string;
}

export interface GpioEvent {
  pin: number;
  value: number;
  timestamp: number;
}

// ==================== PWM Types ====================

export interface PwmConfig {
  channel: number;
  chip: number;
  frequency: number;
  duty_cycle: number;
  polarity: 'normal' | 'inverted';
  enabled: boolean;
}

export interface PwmChannelInfo {
  channel: number;
  chip: number;
  frequency: number;
  duty_cycle: number;
  polarity: 'normal' | 'inverted';
  enabled: boolean;
  period_ns: number;
  duty_cycle_ns: number;
}

export interface PwmWaveform {
  name: string;
  frequency_steps: number[];
  duty_steps: number[];
  step_duration_ms: number;
  cycles: number;
}

// ==================== Log Types ====================

export interface LogEntry {
  id: string;
  timestamp: number;
  level: 'debug' | 'info' | 'warn' | 'error';
  source: string;
  message: string;
  data?: unknown;
}

export interface LogFilter {
  levels: string[];
  sources: string[];
  search: string;
  startTime?: number;
  endTime?: number;
}

// ==================== App Types ====================

export interface AppConfig {
  theme: 'light' | 'dark' | 'system';
  language: string;
  auto_save_interval: number;
  serial_buffer_size: number;
  max_log_entries: number;
  hardware_acceleration: boolean;
}

export interface SystemInfo {
  version: string;
  platform: string;
  arch: string;
}

// ==================== API Types ====================

export interface ApiResponse<T> {
  success: boolean;
  data?: T;
  error?: string;
  error_code?: string;
}

// ==================== UI Types ====================

export type ViewType = 'serial' | 'gpio' | 'pwm' | 'log' | 'settings';

export interface ThemeColors {
  primary: string;
  secondary: string;
  background: string;
  surface: string;
  error: string;
  warning: string;
  success: string;
  info: string;
  text: string;
  textSecondary: string;
  border: string;
}

export interface ToastMessage {
  id: string;
  type: 'success' | 'error' | 'warning' | 'info';
  message: string;
  duration?: number;
}

// ==================== Chart Types ====================

export interface ChartDataPoint {
  timestamp: number;
  value: number;
  label?: string;
}

export interface ChartSeries {
  name: string;
  data: ChartDataPoint[];
  color?: string;
}

// ==================== Device Types ====================

export interface DeviceInfo {
  id: string;
  name: string;
  type: 'serial' | 'gpio' | 'pwm' | 'i2c' | 'spi';
  connected: boolean;
  metadata: Record<string, string>;
}