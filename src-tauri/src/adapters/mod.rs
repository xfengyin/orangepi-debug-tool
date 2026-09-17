pub mod generic_linux;
pub mod mock;
pub mod orangepi_zero3;
pub mod registry;
pub mod traits;

pub use crate::observability::health::HealthStatus;
pub use registry::DeviceAdapterRegistry;
pub use traits::*;

use std::sync::Arc;

pub fn create_default_registry() -> DeviceAdapterRegistry {
    DeviceAdapterRegistry::new()
}

pub fn create_with_adapters() -> DeviceAdapterRegistry {
    let mut registry = DeviceAdapterRegistry::new();

    let mock = Arc::new(mock::MockAdapter::new(0, 0.0));
    registry.register_serial(mock.clone());
    registry.register_gpio(mock.clone());
    registry.register_pwm(mock.clone());

    #[cfg(feature = "hardware-support")]
    {
        let zero3 = Arc::new(orangepi_zero3::OrangePiZero3Adapter::new());
        registry
            .register(zero3.clone())
            .expect("register orangepi_zero3 adapter");
        let linux = Arc::new(generic_linux::GenericLinuxAdapter::new());
        registry
            .register(linux.clone())
            .expect("register generic_linux adapter");
    }

    #[cfg(not(feature = "hardware-support"))]
    {
        registry.register(mock).expect("register mock adapter");
    }

    registry
}
