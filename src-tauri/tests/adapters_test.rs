use orangepi_debug_tool_lib::serial::{SerialAdapter, SerialConfig, SerialHandle};
use orangepi_debug_tool_lib::network::{NetworkAdapter, TcpConfig};
use orangepi_debug_tool_lib::mock::{MockSerialAdapter, MockNetworkAdapter};
use orangepi_debug_tool_lib::adapters::serial::native::CircularBuffer;

#[tokio::test]
async fn test_mock_serial_list_ports() {
    let adapter = MockSerialAdapter::new();
    let ports = adapter.list_ports().await.unwrap();
    
    assert_eq!(ports.len(), 2);
    assert_eq!(ports[0].name, "COM1");
    assert_eq!(ports[1].name, "COM2");
}

#[tokio::test]
async fn test_mock_serial_connect_disconnect() {
    let adapter = MockSerialAdapter::new();
    
    let config = SerialConfig {
        port_name: "COM1".to_string(),
        baudrate: 115200,
        data_bits: 8,
        stop_bits: 1,
        parity: "none".to_string(),
    };
    
    let handle = adapter.connect(&config).await.unwrap();
    assert!(!handle.id.is_empty());
    assert_eq!(handle.config.port_name, "COM1");
    
    adapter.disconnect(&handle).await.unwrap();
}

#[tokio::test]
async fn test_mock_serial_write() {
    let adapter = MockSerialAdapter::new().with_delay(1);
    
    let config = SerialConfig {
        port_name: "COM1".to_string(),
        baudrate: 115200,
        data_bits: 8,
        stop_bits: 1,
        parity: "none".to_string(),
    };
    
    let handle = adapter.connect(&config).await.unwrap();
    
    let data = b"Hello, Serial!";
    let written = adapter.write(&handle, data).await.unwrap();
    
    assert_eq!(written, data.len());
    
    adapter.disconnect(&handle).await.unwrap();
}

#[tokio::test]
async fn test_mock_serial_read() {
    let adapter = MockSerialAdapter::new()
        .with_delay(1)
        .with_response(b"Test data".to_vec());
    
    let config = SerialConfig {
        port_name: "COM1".to_string(),
        baudrate: 115200,
        data_bits: 8,
        stop_bits: 1,
        parity: "none".to_string(),
    };
    
    let handle = adapter.connect(&config).await.unwrap();
    
    let mut buffer = [0u8; 64];
    let bytes_read = adapter.read(&handle, &mut buffer).await.unwrap();
    
    assert_eq!(bytes_read, 9);
    assert_eq!(&buffer[..bytes_read], b"Test data");
    
    adapter.disconnect(&handle).await.unwrap();
}

#[tokio::test]
async fn test_circular_buffer_basic() {
    let mut buffer = CircularBuffer::new(10);
    
    assert_eq!(buffer.available(), 0);
    assert_eq!(buffer.capacity(), 10);
    
    let data = [1, 2, 3, 4, 5];
    let written = buffer.write(&data);
    
    assert_eq!(written, 5);
    assert_eq!(buffer.available(), 5);
    
    let mut read_buffer = [0u8; 5];
    let read = buffer.read(&mut read_buffer);
    
    assert_eq!(read, 5);
    assert_eq!(&read_buffer[..], &[1, 2, 3, 4, 5]);
    assert_eq!(buffer.available(), 0);
}

#[tokio::test]
async fn test_circular_buffer_overflow() {
    let mut buffer = CircularBuffer::new(5);
    
    let data = [1, 2, 3, 4, 5, 6, 7];
    let written = buffer.write(&data);
    
    assert_eq!(written, 5);
    assert_eq!(buffer.available(), 5);
    
    let mut read_buffer = [0u8; 5];
    let read = buffer.read(&mut read_buffer);
    
    assert_eq!(read, 5);
}

#[tokio::test]
async fn test_circular_buffer_wrap_around() {
    let mut buffer = CircularBuffer::new(8);
    
    buffer.write(&[1, 2, 3, 4]).unwrap();
    let mut tmp = [0u8; 2];
    buffer.read(&mut tmp);
    
    buffer.write(&[5, 6, 7, 8, 9]).unwrap();
    
    let mut read_buffer = [0u8; 6];
    let read = buffer.read(&mut read_buffer);
    
    assert_eq!(read, 6);
    assert_eq!(&read_buffer[..], &[3, 4, 5, 6, 7, 8]);
}

#[tokio::test]
async fn test_circular_buffer_clear() {
    let mut buffer = CircularBuffer::new(10);
    
    buffer.write(&[1, 2, 3, 4, 5]).unwrap();
    assert_eq!(buffer.available(), 5);
    
    buffer.clear();
    assert_eq!(buffer.available(), 0);
    
    let written = buffer.write(&[6, 7, 8]).unwrap();
    assert_eq!(written, 3);
    assert_eq!(buffer.available(), 3);
}

#[tokio::test]
async fn test_mock_network_tcp_server() {
    let adapter = MockNetworkAdapter::new().with_delay(1);
    
    let server = adapter.create_tcp_server(8080).await.unwrap();
    assert_eq!(server.port, 8080);
    assert!(!server.id.is_empty());
}

#[tokio::test]
async fn test_mock_network_tcp_connect() {
    let adapter = MockNetworkAdapter::new().with_delay(1);
    
    let config = TcpConfig::new("localhost".to_string(), 8080);
    
    let connection = adapter.connect_tcp(&config).await.unwrap();
    assert_eq!(connection.peer_addr, "localhost:8080");
    assert!(!connection.id.is_empty());
    
    adapter.disconnect_tcp(&connection).await.unwrap();
}

#[tokio::test]
async fn test_mock_network_tcp_send_receive() {
    let adapter = MockNetworkAdapter::new().with_delay(1);
    
    let config = TcpConfig::new("localhost".to_string(), 8080);
    let connection = adapter.connect_tcp(&config).await.unwrap();
    
    let data = b"Hello, TCP!";
    adapter.send_tcp(&connection, data).await.unwrap();
    
    let mut buffer = [0u8; 64];
    let bytes_read = adapter.receive_tcp(&connection, &mut buffer).await.unwrap();
    
    assert_eq!(bytes_read, 13);
    assert_eq!(&buffer[..bytes_read], b"mock response");
    
    adapter.disconnect_tcp(&connection).await.unwrap();
}

#[tokio::test]
async fn test_mock_network_udp_socket() {
    let adapter = MockNetworkAdapter::new().with_delay(1);
    
    let socket = adapter.create_udp(8081).await.unwrap();
    assert_eq!(socket.local_port, 8081);
    assert!(!socket.id.is_empty());
}

#[tokio::test]
async fn test_mock_network_udp_send_receive() {
    let adapter = MockNetworkAdapter::new().with_delay(1);
    
    let socket = adapter.create_udp(8081).await.unwrap();
    
    let data = b"Hello, UDP!";
    adapter.send_udp(&socket, data, "localhost", 8082).await.unwrap();
    
    let mut buffer = [0u8; 64];
    let (bytes_read, addr, port) = adapter.receive_udp(&socket, &mut buffer).await.unwrap();
    
    assert_eq!(bytes_read, 16);
    assert_eq!(&buffer[..bytes_read], b"mock udp response");
    assert_eq!(addr, "127.0.0.1");
    assert_eq!(port, 12345);
}

#[tokio::test]
async fn test_mock_network_error_simulation() {
    let adapter = MockNetworkAdapter::new()
        .with_delay(1)
        .simulate_error(true);
    
    let result = adapter.create_tcp_server(8080).await;
    assert!(result.is_err());
    
    let config = TcpConfig::new("localhost".to_string(), 8080);
    let result = adapter.connect_tcp(&config).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_tcp_config_address() {
    let config = TcpConfig::new("192.168.1.1".to_string(), 8080);
    assert_eq!(config.address(), "192.168.1.1:8080");
}
