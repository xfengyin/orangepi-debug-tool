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
    
    #[cfg(feature = "hardware-support")]
    {
        registry.register(Arc::new(orangepi_zero3::OrangePiZero3Adapter::new()));
        registry.register(Arc::new(generic_linux::GenericLinuxAdapter::new()));
    }
    
    #[cfg(not(feature = "hardware-support"))]
    {
        registry.register(Arc::new(mock::MockAdapter::new()));
    }
    
    registry
}
