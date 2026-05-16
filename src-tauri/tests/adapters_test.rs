use std::sync::Arc;
use test_lib::adapters::{MockSerialAdapter, TokioNetworkAdapter, TcpServerConfig, NetworkAdapter, SerialAdapter, SerialConfig};

#[cfg(test)]
mod mock_serial_tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_list_ports() {
        let adapter = MockSerialAdapter::new();
        let ports = adapter.list_ports().await.unwrap();
        assert!(!ports.is_empty());
        assert_eq!(ports.len(), 3);
    }

    #[tokio::test]
    async fn test_mock_connect_disconnect() {
        let adapter = MockSerialAdapter::new();
        let config = SerialConfig {
            port_name: "/dev/ttyUSB0".to_string(),
            baudrate: 115200,
            data_bits: 8,
            stop_bits: 1,
            parity: "none".to_string(),
        };

        let handle = adapter.connect(&config).await.unwrap();
        assert!(!handle.id.is_empty());
        assert_eq!(handle.port_name, "/dev/ttyUSB0");

        adapter.disconnect(&handle).await.unwrap();
    }

    #[tokio::test]
    async fn test_mock_read_write() {
        let adapter = MockSerialAdapter::new();
        let config = SerialConfig {
            port_name: "/dev/ttyUSB0".to_string(),
            baudrate: 115200,
            data_bits: 8,
            stop_bits: 1,
            parity: "none".to_string(),
        };

        let handle = adapter.connect(&config).await.unwrap();

        let data = vec![0x01, 0x02, 0x03, 0x04];
        let n = adapter.write(&handle, &data).await.unwrap();
        assert_eq!(n, 4);

        let mut buffer = vec![0u8; 4];
        let n = adapter.read(&handle, &mut buffer).await.unwrap();
        assert_eq!(n, 4);
        assert_eq!(buffer, data);

        adapter.disconnect(&handle).await.unwrap();
    }

    #[tokio::test]
    async fn test_mock_multiple_connections() {
        let adapter = MockSerialAdapter::new();
        
        let config1 = SerialConfig {
            port_name: "/dev/ttyUSB0".to_string(),
            baudrate: 115200,
            data_bits: 8,
            stop_bits: 1,
            parity: "none".to_string(),
        };
        
        let config2 = SerialConfig {
            port_name: "/dev/ttyUSB1".to_string(),
            baudrate: 9600,
            data_bits: 8,
            stop_bits: 1,
            parity: "none".to_string(),
        };

        let handle1 = adapter.connect(&config1).await.unwrap();
        let handle2 = adapter.connect(&config2).await.unwrap();

        assert_ne!(handle1.id, handle2.id);

        adapter.disconnect(&handle1).await.unwrap();
        adapter.disconnect(&handle2).await.unwrap();
    }

    #[tokio::test]
    async fn test_mock_id() {
        let adapter = MockSerialAdapter::new();
        assert_eq!(adapter.id(), "mock-serial");
    }
}

#[cfg(test)]
mod network_tests {
    use super::*;

    #[tokio::test]
    async fn test_tcp_server_create() {
        let adapter = TokioNetworkAdapter::new();
        let config = TcpServerConfig {
            port: 19999,
            max_connections: 10,
        };

        let server = adapter.create_tcp_server(config).await.unwrap();
        assert!(server.local_addr.port() == 19999);
        assert!(!server.id.is_empty());

        adapter.close_tcp_server(&server).await.unwrap();
    }

    #[tokio::test]
    async fn test_tcp_connect_disconnect() {
        let adapter = TokioNetworkAdapter::new();
        let server_config = TcpServerConfig {
            port: 19998,
            max_connections: 10,
        };

        let server = adapter.create_tcp_server(server_config).await.unwrap();

        let conn = adapter.connect_tcp("127.0.0.1", 19998).await.unwrap();
        
        assert!(!conn.id.is_empty());

        adapter.disconnect_tcp(&conn).await.unwrap();
        adapter.close_tcp_server(&server).await.unwrap();
    }

    #[tokio::test]
    async fn test_tcp_send_receive() {
        let adapter = TokioNetworkAdapter::new();
        let server_config = TcpServerConfig {
            port: 19997,
            max_connections: 10,
        };

        let server = adapter.create_tcp_server(server_config).await.unwrap();

        let client_conn = adapter.connect_tcp("127.0.0.1", 19997).await.unwrap();
        let server_conn = adapter.accept_tcp_connection(&server).await.unwrap();

        let data = b"Hello, TCP!";
        adapter.send_tcp(&client_conn, data).await.unwrap();

        let mut buffer = vec![0u8; 1024];
        let n = adapter.receive_tcp(&server_conn, &mut buffer).await.unwrap();
        assert_eq!(&buffer[..n], data);

        let response = b"Response!";
        adapter.send_tcp(&server_conn, response).await.unwrap();
        
        let mut recv_buffer = vec![0u8; 1024];
        let n = adapter.receive_tcp(&client_conn, &mut recv_buffer).await.unwrap();
        assert_eq!(&recv_buffer[..n], response);

        adapter.disconnect_tcp(&client_conn).await.unwrap();
        adapter.disconnect_tcp(&server_conn).await.unwrap();
        adapter.close_tcp_server(&server).await.unwrap();
    }

    #[tokio::test]
    async fn test_udp_create_send_receive() {
        let adapter = TokioNetworkAdapter::new();

        let socket = adapter.connect_udp(19996, "127.0.0.1", 19996).await.unwrap();
        assert!(!socket.id.is_empty());

        let data = b"Hello, UDP!";
        adapter.send_udp(&socket, data, "127.0.0.1", 19996).await.unwrap();

        let mut buffer = vec![0u8; 1024];
        let (n, _from_addr, from_port) = adapter.receive_udp(&socket, &mut buffer).await.unwrap();
        assert_eq!(&buffer[..n], data);
        assert_eq!(from_port, 19996);

        adapter.disconnect_udp(&socket).await.unwrap();
    }
}
