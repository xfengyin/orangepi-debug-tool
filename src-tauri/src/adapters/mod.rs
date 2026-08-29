pub mod traits;
pub mod registry;
pub mod orangepi_zero3;
pub mod generic_linux;
pub mod mock;

pub use traits::*;
pub use registry::DeviceAdapterRegistry;
pub use crate::observability::health::HealthStatus;

use std::sync::Arc;

pub fn create_default_registry() -> DeviceAdapterRegistry {
    DeviceAdapterRegistry::new()
}

pub fn create_with_adapters() -> DeviceAdapterRegistry {
    let mut registry = DeviceAdapterRegistry::new();
    
    let mock = Arc::new(mock::MockAdapter::new());
    registry.register_serial(mock.clone());
    registry.register_gpio(mock.clone());
    registry.register_pwm(mock.clone());

    #[cfg(feature = "hardware-support")]
    {
        let zero3 = Arc::new(orangepi_zero3::OrangePiZero3Adapter::new());
        registry.register(zero3.clone());
        let linux = Arc::new(generic_linux::GenericLinuxAdapter::new());
        registry.register(linux.clone());
    }

    #[cfg(not(feature = "hardware-support"))]
    {
        registry.register(mock);
    }
    
    registry
}
