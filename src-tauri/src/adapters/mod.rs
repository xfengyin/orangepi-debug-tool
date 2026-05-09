pub mod traits;
pub mod registry;
pub mod orangepi_zero3;
pub mod generic_linux;
pub mod mock;

pub use traits::*;
pub use registry::DeviceAdapterRegistry;

use std::sync::Arc;

pub fn create_default_registry() -> DeviceAdapterRegistry {
    DeviceAdapterRegistry::new()
}

pub fn create_with_adapters() -> DeviceAdapterRegistry {
    let mut registry = DeviceAdapterRegistry::new();
    
    #[cfg(feature = "hardware-support")]
    {
        registry.register(orangepi_zero3::OrangePiZero3Adapter::new());
        registry.register(generic_linux::GenericLinuxAdapter::new());
    }
    
    #[cfg(not(feature = "hardware-support"))]
    {
        registry.register(mock::MockAdapter::new());
    }
    
    registry
}
