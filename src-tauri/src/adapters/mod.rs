pub mod serial;
pub mod network;
pub mod network_impl;
pub mod mock;

#[cfg(feature = "native-serial")]
pub use serial::NativeSerialAdapter;

pub use network::{NetworkAdapter, TcpHandle, UdpHandle, TcpServerHandle, TcpServerConfig, DeviceError, DeviceResult};
pub use network_impl::TokioNetworkAdapter;
pub use serial::{SerialAdapter, SerialConfig, SerialHandle, SerialPortInfo, SerialError, SerialResult};
pub use mock::MockSerialAdapter;
