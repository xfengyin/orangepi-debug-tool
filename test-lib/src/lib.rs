pub mod adapters {
    pub mod mock;
    pub mod network;
    pub mod network_impl;
    pub mod serial;
    
    pub use network::{DeviceError, DeviceResult, NetworkAdapter, TcpConfig, TcpHandle, TcpServerConfig, TcpServerHandle, UdpHandle};
    pub use network_impl::TokioNetworkAdapter;
    pub use serial::{SerialAdapter, SerialConfig, SerialHandle, SerialPortInfo, SerialError, SerialResult, CircularBuffer};
    pub use mock::MockSerialAdapter;
}

pub mod services {
    pub mod log_service;
    pub mod network_service;
    pub mod serial_service;
    
    pub use log_service::LogService;
    pub use network_service::{NetworkService, NetworkServiceError, TcpConnectionInfo, UdpSessionInfo, TcpServerInfo};
    pub use serial_service::{SerialService, SerialServiceError, SerialStats};
}
