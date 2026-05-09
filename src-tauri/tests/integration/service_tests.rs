#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use parking_lot::Mutex;
    use crate::adapters::{MockAdapter, DeviceAdapterRegistry, SerialConfig};
    use crate::services::{SerialService, GpioService, PwmService};
    use crate::config::{SerialDeviceConfig, GpioDeviceConfig, PwmDeviceConfig};

    #[tokio::test]
    async fn test_serial_service_connect_disconnect() {
        let registry = Arc::new(Mutex::new(DeviceAdapterRegistry::new()));
        registry.lock().register(MockAdapter::new());
        
        let config = SerialDeviceConfig::default();
        let service = SerialService::new(registry.clone(), &config);
        
        service.initialize().await.unwrap();
        
        let ports = service.list_ports().await.unwrap();
        assert_eq!(ports.len(), 2);
        
        let connection_id = service.connect("/dev/ttyUSB0", 115200).await.unwrap();
        assert!(!connection_id.is_empty());
        
        service.disconnect(&connection_id).await.unwrap();
        
        service.shutdown().await.unwrap();
    }

    #[tokio::test]
    async fn test_serial_service_send_receive() {
        let registry = Arc::new(Mutex::new(DeviceAdapterRegistry::new()));
        registry.lock().register(MockAdapter::new());
        
        let config = SerialDeviceConfig::default();
        let service = SerialService::new(registry.clone(), &config);
        
        service.initialize().await.unwrap();
        
        let connection_id = service.connect("/dev/ttyUSB0", 115200).await.unwrap();
        
        let data = b"Hello, OrangePi!";
        let written = service.send(&connection_id, data).await.unwrap();
        assert_eq!(written, data.len());
        
        let mut buffer = [0u8; 64];
        let read = service.receive(&connection_id, &mut buffer, 100).await.unwrap();
        assert!(read > 0);
        
        service.disconnect(&connection_id).await.unwrap();
        service.shutdown().await.unwrap();
    }

    #[tokio::test]
    async fn test_gpio_service_export_unexport() {
        let registry = Arc::new(Mutex::new(DeviceAdapterRegistry::new()));
        registry.lock().register(MockAdapter::new());
        
        let config = GpioDeviceConfig::default();
        let service = GpioService::new(registry.clone(), &config);
        
        service.initialize().await.unwrap();
        
        let pins = service.list_pins().await.unwrap();
        assert_eq!(pins.len(), 4);
        
        service.export_pin(7).await.unwrap();
        service.unexport_pin(7).await.unwrap();
        
        service.shutdown().await.unwrap();
    }

    #[tokio::test]
    async fn test_gpio_service_read_write() {
        let registry = Arc::new(Mutex::new(DeviceAdapterRegistry::new()));
        registry.lock().register(MockAdapter::new());
        
        let config = GpioDeviceConfig::default();
        let service = GpioService::new(registry.clone(), &config);
        
        service.initialize().await.unwrap();
        
        service.export_pin(3).await.unwrap();
        
        service.write_pin(3, 1).await.unwrap();
        let value = service.read_pin(3).await.unwrap();
        assert_eq!(value, 1);
        
        service.write_pin(3, 0).await.unwrap();
        let value = service.read_pin(3).await.unwrap();
        assert_eq!(value, 0);
        
        service.unexport_pin(3).await.unwrap();
        service.shutdown().await.unwrap();
    }

    #[tokio::test]
    async fn test_gpio_service_batch_operations() {
        let registry = Arc::new(Mutex::new(DeviceAdapterRegistry::new()));
        registry.lock().register(MockAdapter::new());
        
        let config = GpioDeviceConfig::default();
        let service = GpioService::new(registry.clone(), &config);
        
        service.initialize().await.unwrap();
        
        service.export_pin(3).await.unwrap();
        service.export_pin(5).await.unwrap();
        
        let results = service.batch_read(&[3, 5]).await.unwrap();
        assert_eq!(results.len(), 2);
        assert!(results.contains_key(&3));
        assert!(results.contains_key(&5));
        
        service.batch_write(&[(3, 1), (5, 1)]).await.unwrap();
        
        let results = service.batch_read(&[3, 5]).await.unwrap();
        assert_eq!(results.get(&3), Some(&1));
        assert_eq!(results.get(&5), Some(&1));
        
        service.shutdown().await.unwrap();
    }

    #[tokio::test]
    async fn test_pwm_service_enable_disable() {
        let registry = Arc::new(Mutex::new(DeviceAdapterRegistry::new()));
        registry.lock().register(MockAdapter::new());
        
        let config = PwmDeviceConfig::default();
        let service = PwmService::new(registry.clone(), &config);
        
        service.initialize().await.unwrap();
        
        let channels = service.list_channels().await.unwrap();
        assert_eq!(channels.len(), 2);
        
        service.enable_channel(0).await.unwrap();
        service.disable_channel(0).await.unwrap();
        
        service.shutdown().await.unwrap();
    }

    #[tokio::test]
    async fn test_pwm_service_frequency_duty_cycle() {
        let registry = Arc::new(Mutex::new(DeviceAdapterRegistry::new()));
        registry.lock().register(MockAdapter::new());
        
        let config = PwmDeviceConfig::default();
        let service = PwmService::new(registry.clone(), &config);
        
        service.initialize().await.unwrap();
        
        service.enable_channel(0).await.unwrap();
        
        service.set_frequency(0, 1000).await.unwrap();
        service.set_duty_cycle(0, 50.0).await.unwrap();
        
        let state = service.get_channel_state(0).await.unwrap();
        assert_eq!(state.frequency as u32, 1000);
        assert_eq!(state.duty_cycle, 50.0);
        
        service.disable_channel(0).await.unwrap();
        service.shutdown().await.unwrap();
    }

    #[tokio::test]
    async fn test_pwm_service_servo_control() {
        let registry = Arc::new(Mutex::new(DeviceAdapterRegistry::new()));
        registry.lock().register(MockAdapter::new());
        
        let config = PwmDeviceConfig::default();
        let service = PwmService::new(registry.clone(), &config);
        
        service.initialize().await.unwrap();
        
        service.enable_channel(0).await.unwrap();
        
        service.configure_servo(0, 90.0).await.unwrap();
        
        let state = service.get_channel_state(0).await.unwrap();
        assert!((state.duty_cycle - 7.5).abs() < 0.1);
        
        service.disable_channel(0).await.unwrap();
        service.shutdown().await.unwrap();
    }

    #[tokio::test]
    async fn test_device_adapter_registry() {
        let mut registry = DeviceAdapterRegistry::new();
        
        let adapter = MockAdapter::new();
        registry.register(adapter).unwrap();
        
        assert!(registry.has_adapter("mock"));
        
        let retrieved = registry.get("mock");
        assert!(retrieved.is_some());
        
        registry.unregister("mock");
        assert!(!registry.has_adapter("mock"));
    }

    #[tokio::test]
    async fn test_device_adapter_registry_default_adapters() {
        let mut registry = DeviceAdapterRegistry::new();
        
        registry.register(MockAdapter::new()).unwrap();
        
        let serial_adapter = registry.get_default_serial();
        assert!(serial_adapter.is_some());
        
        let gpio_adapter = registry.get_default_gpio();
        assert!(gpio_adapter.is_some());
        
        let pwm_adapter = registry.get_default_pwm();
        assert!(pwm_adapter.is_some());
    }

    #[tokio::test]
    async fn test_device_adapter_auto_detect() {
        let registry = Arc::new(Mutex::new(DeviceAdapterRegistry::new()));
        registry.lock().register(MockAdapter::new());
        
        let device_info = registry.lock().auto_detect().await.unwrap();
        assert_eq!(device_info.id, "mock");
    }

    #[tokio::test]
    async fn test_multiple_connections() {
        let registry = Arc::new(Mutex::new(DeviceAdapterRegistry::new()));
        registry.lock().register(MockAdapter::new());
        
        let config = SerialDeviceConfig::default();
        let service = SerialService::new(registry.clone(), &config);
        
        service.initialize().await.unwrap();
        
        let conn1 = service.connect("/dev/ttyUSB0", 115200).await.unwrap();
        let conn2 = service.connect("/dev/ttyUSB1", 57600).await.unwrap();
        
        assert_ne!(conn1, conn2);
        
        let connections = service.list_connections().await;
        assert_eq!(connections.len(), 2);
        
        service.disconnect(&conn1).await.unwrap();
        service.disconnect(&conn2).await.unwrap();
        
        service.shutdown().await.unwrap();
    }
}
