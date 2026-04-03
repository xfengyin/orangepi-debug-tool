import '@testing-library/jest-dom';
import { vi } from 'vitest';

// Mock Tauri APIs
vi.mock('@tauri-apps/api', () => ({
  invoke: vi.fn(),
  convertFileSrc: vi.fn(),
  transformCallback: vi.fn(),
}));

vi.mock('@tauri-apps/api/tauri', () => ({
  invoke: vi.fn(),
}));

vi.mock('@tauri-apps/api/event', () => ({
  listen: vi.fn(),
  emit: vi.fn(),
}));

vi.mock('@tauri-apps/api/window', () => ({
  getCurrent: vi.fn(() => ({
    listen: vi.fn(),
  })),
}));

// Mock matchMedia
Object.defineProperty(window, 'matchMedia', {
  writable: true,
  value: vi.fn().mockImplementation((query: string) => ({
    matches: false,
    media: query,
    onchange: null,
    addListener: vi.fn(),
    removeListener: vi.fn(),
    addEventListener: vi.fn(),
    removeEventListener: vi.fn(),
    dispatchEvent: vi.fn(),
  })),
});