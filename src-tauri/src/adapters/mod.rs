pub mod serial;
pub mod network;
pub mod network_impl;

pub use network::{NetworkAdapter, TcpHandle, UdpHandle, TcpServerHandle, TcpConnection, TcpServerConfig, DeviceError, DeviceResult};
pub use network_impl::TokioNetworkAdapter;
pub use serial::{SerialAdapter, SerialConfig, SerialHandle, SerialPortInfo};
