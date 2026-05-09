#[cfg(test)]
mod tests {
    use crate::adapters::{MockAdapter, MockConfig, MockDataPattern};
    use crate::adapters::{DeviceCapability, SerialConfig, GpioDirection, GpioTrigger, GpioPull};
    use std::collections::HashSet;

    #[tokio::test]
    async fn test_mock_adapter_health_check() {
        let adapter = MockAdapter::new();
        let health = adapter.health_check().await.unwrap();
        assert!(health.is_healthy());
    }

    #[tokio::test]
    async fn test_mock_adapter_capabilities() {
        let adapter = MockAdapter::new();
        let caps = adapter.capabilities();
        
        assert!(caps.contains(&DeviceCapability::Serial));
        assert!(caps.contains(&DeviceCapability::Gpio));
        assert!(caps.contains(&DeviceCapability::Pwm));
    }

    #[tokio::test]
    async fn test_mock_serial_list_ports() {
        let adapter = MockAdapter::new();
        let ports = adapter.list_ports().await.unwrap();
        
        assert_eq!(ports.len(), 2);
        assert_eq!(ports[0].port_name, "/dev/ttyUSB0");
        assert_eq!(ports[1].port_name, "/dev/ttyUSB1");
    }

    #[tokio::test]
    async fn test_mock_serial_connect_disconnect() {
        let adapter = MockAdapter::new();
        
        let config = SerialConfig {
            port_name: "/dev/ttyUSB0".to_string(),
            baud_rate: 115200,
            data_bits: 8,
            parity: "none".to_string(),
            stop_bits: 1,
            flow_control: "none".to_string(),
            read_timeout_ms: 1000,
            write_timeout_ms: 1000,
        };
        
        let handle = adapter.connect(config.clone()).await.unwrap();
        assert_eq!(handle.port_name, "/dev/ttyUSB0");
        
        adapter.disconnect(handle).await.unwrap();
    }

    #[tokio::test]
    async fn test_mock_serial_read_write() {
        let adapter = MockAdapter::new();
        
        let config = SerialConfig::default();
        let handle = adapter.connect(config.clone()).await.unwrap();
        
        let write_data = b"Hello, OrangePi!";
        let written = adapter.write(&handle, write_data).await.unwrap();
        assert_eq!(written, write_data.len());
        
        let mut read_buffer = [0u8; 64];
        let read = adapter.read(&handle, &mut read_buffer).await.unwrap();
        assert!(read > 0);
        
        adapter.disconnect(handle).await.unwrap();
    }

    #[tokio::test]
    async fn test_mock_gpio_list_pins() {
        let adapter = MockAdapter::new();
        let pins = adapter.list_pins().await.unwrap();
        
        assert_eq!(pins.len(), 4);
        assert_eq!(pins[0].pin, 3);
        assert_eq!(pins[0].name, "GPIO3");
    }

    #[tokio::test]
    async fn test_mock_gpio_export_unexport() {
        let adapter = MockAdapter::new();
        
        adapter.export_pin(7).await.unwrap();
        adapter.unexport_pin(7).await.unwrap();
    }

    #[tokio::test]
    async fn test_mock_gpio_read_write() {
        let adapter = MockAdapter::new();
        
        adapter.export_pin(3).await.unwrap();
        
        adapter.write_pin(3, 1).await.unwrap();
        let value = adapter.read_pin(3).await.unwrap();
        assert_eq!(value, 1);
        
        adapter.write_pin(3, 0).await.unwrap();
        let value = adapter.read_pin(3).await.unwrap();
        assert_eq!(value, 0);
        
        adapter.unexport_pin(3).await.unwrap();
    }

    #[tokio::test]
    async fn test_mock_gpio_direction() {
        let adapter = MockAdapter::new();
        
        adapter.export_pin(5).await.unwrap();
        adapter.set_direction(5, GpioDirection::Input).await.unwrap();
        adapter.set_direction(5, GpioDirection::Output).await.unwrap();
        adapter.unexport_pin(5).await.unwrap();
    }

    #[tokio::test]
    async fn test_mock_gpio_interrupt() {
        let adapter = MockAdapter::new();
        
        adapter.export_pin(11).await.unwrap();
        adapter.enable_interrupt(11, GpioTrigger::Rising).await.unwrap();
        adapter.disable_interrupt(11).await.unwrap();
        adapter.unexport_pin(11).await.unwrap();
    }

    #[tokio::test]
    async fn test_mock_pwm_list_channels() {
        let adapter = MockAdapter::new();
        let channels = adapter.list_channels().await.unwrap();
        
        assert_eq!(channels.len(), 2);
        assert_eq!(channels[0].channel, 0);
        assert_eq!(channels[1].channel, 1);
    }

    #[tokio::test]
    async fn test_mock_pwm_enable_disable() {
        let adapter = MockAdapter::new();
        
        adapter.enable_channel(0).await.unwrap();
        adapter.disable_channel(0).await.unwrap();
    }

    #[tokio::test]
    async fn test_mock_pwm_set_frequency_duty() {
        let adapter = MockAdapter::new();
        
        adapter.enable_channel(0).await.unwrap();
        
        adapter.set_frequency(0, 1000).await.unwrap();
        adapter.set_duty_cycle(0, 50.0).await.unwrap();
        
        adapter.set_frequency(0, 2000).await.unwrap();
        adapter.set_duty_cycle(0, 75.0).await.unwrap();
        
        adapter.disable_channel(0).await.unwrap();
    }

    #[tokio::test]
    async fn test_mock_adapter_with_delay() {
        let config = MockConfig {
            simulate_delay_ms: 100,
            inject_errors: false,
            error_rate: 0.0,
            data_pattern: MockDataPattern::Fixed,
        };
        
        let adapter = MockAdapter::with_config(config);
        let start = std::time::Instant::now();
        adapter.health_check().await.unwrap();
        let elapsed = start.elapsed().as_millis() as u64;
        assert!(elapsed >= 100);
    }

    #[tokio::test]
    async fn test_mock_adapter_with_error_injection() {
        let config = MockConfig {
            simulate_delay_ms: 10,
            inject_errors: true,
            error_rate: 1.0,
            data_pattern: MockDataPattern::Incrementing,
        };
        
        let adapter = MockAdapter::with_config(config);
        
        let config = SerialConfig::default();
        let result = adapter.connect(config.clone()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_mock_adapter_reset() {
        let adapter = MockAdapter::new();
        
        adapter.export_pin(3).await.unwrap();
        adapter.write_pin(3, 1).await.unwrap();
        
        adapter.reset().await;
        
        let values = adapter.get_gpio_values().await;
        assert!(values.is_empty());
    }

    #[test]
    fn test_mock_config_default() {
        let config = MockConfig::default();
        assert_eq!(config.simulate_delay_ms, 10);
        assert!(!config.inject_errors);
        assert_eq!(config.error_rate, 0.0);
    }

    #[test]
    fn test_mock_data_patterns() {
        let config = MockConfig {
            simulate_delay_ms: 0,
            inject_errors: false,
            error_rate: 0.0,
            data_pattern: MockDataPattern::Incrementing,
        };
        let adapter = MockAdapter::with_config(config);
        assert_eq!(adapter.config.data_pattern, MockDataPattern::Incrementing);
    }
}
