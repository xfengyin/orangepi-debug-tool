import { describe, it, expect, vi, beforeEach } from 'vitest';
import { useSerialStore } from '../../src/stores/serialStore';

// Mock Tauri API
vi.mock('@tauri-apps/api/tauri', () => ({
  invoke: vi.fn(),
}));

describe('Serial Store', () => {
  beforeEach(() => {
    // Reset store state
    const store = useSerialStore.getState();
    store.setPorts([]);
    store.setSelectedPort(null);
    store.setConfig({
      port_name: '',
      baud_rate: 115200,
      data_bits: 8,
      parity: 'none',
      stop_bits: 1,
      flow_control: 'none',
      read_timeout: 1000,
      write_timeout: 1000,
    });
    store.clearData();
  });

  it('should set ports', () => {
    const store = useSerialStore.getState();
    const mockPorts = [
      { port_name: '/dev/ttyUSB0', port_type: 'USB' },
      { port_name: '/dev/ttyUSB1', port_type: 'USB' },
    ];
    
    store.setPorts(mockPorts as any);
    
    expect(useSerialStore.getState().ports).toHaveLength(2);
    expect(useSerialStore.getState().ports[0].port_name).toBe('/dev/ttyUSB0');
  });

  it('should update config', () => {
    const store = useSerialStore.getState();
    store.setConfig({ baud_rate: 9600 });
    
    expect(useSerialStore.getState().config.baud_rate).toBe(9600);
    expect(useSerialStore.getState().config.data_bits).toBe(8); // unchanged
  });

  it('should add data to buffer', () => {
    const store = useSerialStore.getState();
    const packet = {
      timestamp: Date.now(),
      data: [0x01, 0x02, 0x03],
      is_rx: true,
    };
    
    store.addData(packet);
    
    expect(useSerialStore.getState().dataBuffer).toHaveLength(1);
    expect(useSerialStore.getState().dataBuffer[0].data).toEqual([0x01, 0x02, 0x03]);
    expect(useSerialStore.getState().status.rx_bytes).toBe(3);
  });

  it('should limit buffer size', () => {
    const store = useSerialStore.getState();
    
    // Add more than 10000 entries
    for (let i = 0; i < 10010; i++) {
      store.addData({
        timestamp: Date.now(),
        data: [i],
        is_rx: true,
      });
    }
    
    expect(useSerialStore.getState().dataBuffer.length).toBeLessThanOrEqual(10000);
  });
});