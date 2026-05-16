use std::sync::Arc;
use tempfile::TempDir;
use test_lib::adapters::MockSerialAdapter;
use test_lib::adapters::TokioNetworkAdapter;
use test_lib::services::{SerialService, NetworkService, LogService};
use test_lib::adapters::SerialConfig;

#[cfg(test)]
mod serial_service_tests {
    use super::*;

    #[tokio::test]
    async fn test_serial_service_list_ports() {
        let mock = Arc::new(MockSerialAdapter::new());
        let service = SerialService::new(mock);

        let ports = service.list_ports().await.unwrap();
        assert!(!ports.is_empty());
        assert_eq!(ports.len(), 3);
    }

    #[tokio::test]
    async fn test_serial_service_connect_disconnect() {
        let mock = Arc::new(MockSerialAdapter::new());
        let service = SerialService::new(mock);

        let config = SerialConfig {
            port_name: "/dev/ttyUSB0".to_string(),
            baudrate: 115200,
            data_bits: 8,
            stop_bits: 1,
            parity: "none".to_string(),
        };

        let id = service.connect("/dev/ttyUSB0".to_string(), config).await.unwrap();
        assert!(!id.is_empty());

        service.disconnect(&id).await.unwrap();
    }

    #[tokio::test]
    async fn test_serial_service_send_receive() {
        let mock = Arc::new(MockSerialAdapter::new());
        let service = SerialService::new(mock);

        let config = SerialConfig {
            port_name: "/dev/ttyUSB0".to_string(),
            baudrate: 115200,
            data_bits: 8,
            stop_bits: 1,
            parity: "none".to_string(),
        };

        let id = service.connect("/dev/ttyUSB0".to_string(), config).await.unwrap();

        let data = vec![0x01, 0x02, 0x03, 0x04];
        service.send(&id, &data).await.unwrap();

        let mut buffer = vec![0u8; 4];
        let n = service.read(&id, &mut buffer).await.unwrap();
        assert_eq!(n, 4);
        assert_eq!(buffer, data);

        service.disconnect(&id).await.unwrap();
    }

    #[tokio::test]
    async fn test_serial_service_stats() {
        let mock = Arc::new(MockSerialAdapter::new());
        let service = SerialService::new(mock);

        let config = SerialConfig {
            port_name: "/dev/ttyUSB0".to_string(),
            baudrate: 115200,
            data_bits: 8,
            stop_bits: 1,
            parity: "none".to_string(),
        };

        let id = service.connect("/dev/ttyUSB0".to_string(), config).await.unwrap();

        let data = vec![0x01, 0x02, 0x03, 0x04];
        service.send(&id, &data).await.unwrap();

        let stats = service.get_stats(&id).await.unwrap();
        assert_eq!(stats.bytes_sent, 4);
        assert_eq!(stats.packets_sent, 1);

        service.disconnect(&id).await.unwrap();
    }

    #[tokio::test]
    async fn test_serial_service_is_connected() {
        let mock = Arc::new(MockSerialAdapter::new());
        let service = SerialService::new(mock);

        let config = SerialConfig {
            port_name: "/dev/ttyUSB0".to_string(),
            baudrate: 115200,
            data_bits: 8,
            stop_bits: 1,
            parity: "none".to_string(),
        };

        let id = service.connect("/dev/ttyUSB0".to_string(), config).await.unwrap();
        assert!(service.is_connected(&id).await);

        service.disconnect(&id).await.unwrap();
        assert!(!service.is_connected(&id).await);
    }

    #[tokio::test]
    async fn test_serial_service_disconnect_unknown_id() {
        let mock = Arc::new(MockSerialAdapter::new());
        let service = SerialService::new(mock);

        let result = service.disconnect("unknown-id").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_serial_service_read_write_multiple() {
        let mock = Arc::new(MockSerialAdapter::new());
        let service = SerialService::new(mock);

        let config = SerialConfig {
            port_name: "/dev/ttyUSB0".to_string(),
            baudrate: 115200,
            data_bits: 8,
            stop_bits: 1,
            parity: "none".to_string(),
        };

        let id = service.connect("/dev/ttyUSB0".to_string(), config).await.unwrap();

        for i in 0..5 {
            let data = vec![i as u8; 10];
            service.send(&id, &data).await.unwrap();

            let mut buffer = vec![0u8; 10];
            let n = service.read(&id, &mut buffer).await.unwrap();
            assert_eq!(n, 10);
            assert_eq!(buffer, data);
        }

        let stats = service.get_stats(&id).await.unwrap();
        assert_eq!(stats.bytes_sent, 50);
        assert_eq!(stats.packets_sent, 5);

        service.disconnect(&id).await.unwrap();
    }
}

#[cfg(test)]
mod network_service_tests {
    use super::*;

    #[tokio::test]
    async fn test_network_service_tcp_server() {
        let adapter = Arc::new(TokioNetworkAdapter::new());
        let service = NetworkService::new(adapter);

        let server_id = service.create_tcp_server(19995, 10).await.unwrap();
        assert!(!server_id.is_empty());

        service.close_tcp_server(&server_id).await.unwrap();
    }

    #[tokio::test]
    async fn test_network_service_tcp_client_server() {
        let adapter = Arc::new(TokioNetworkAdapter::new());
        let service = NetworkService::new(adapter);

        let server_id = service.create_tcp_server(19994, 10).await.unwrap();

        let client_id = service.connect_tcp("127.0.0.1", 19994).await.unwrap();
        let server_conn_id = service.accept_tcp_connection(&server_id).await.unwrap();

        let data = b"Test data";
        service.send_tcp(&client_id, data).await.unwrap();

        let mut buffer = vec![0u8; 1024];
        let n = service.receive_tcp(&server_conn_id, &mut buffer).await.unwrap();
        assert_eq!(&buffer[..n], data);

        service.disconnect_tcp(&client_id).await.unwrap();
        service.disconnect_tcp(&server_conn_id).await.unwrap();
        service.close_tcp_server(&server_id).await.unwrap();
    }

    #[tokio::test]
    async fn test_network_service_list_tcp_servers() {
        let adapter = Arc::new(TokioNetworkAdapter::new());
        let service = NetworkService::new(adapter);

        let server1_id = service.create_tcp_server(19991, 10).await.unwrap();
        let server2_id = service.create_tcp_server(19990, 10).await.unwrap();

        let servers = service.list_tcp_servers().await;
        assert_eq!(servers.len(), 2);

        service.close_tcp_server(&server1_id).await.unwrap();
        service.close_tcp_server(&server2_id).await.unwrap();
    }

    #[tokio::test]
    async fn test_network_service_bidirectional_tcp() {
        let adapter = Arc::new(TokioNetworkAdapter::new());
        let service = NetworkService::new(adapter);

        let server_id = service.create_tcp_server(19989, 10).await.unwrap();
        let client_id = service.connect_tcp("127.0.0.1", 19989).await.unwrap();
        let server_conn_id = service.accept_tcp_connection(&server_id).await.unwrap();

        let client_msg = b"Hello from client";
        service.send_tcp(&client_id, client_msg).await.unwrap();
        
        let mut server_buffer = vec![0u8; 1024];
        let n = service.receive_tcp(&server_conn_id, &mut server_buffer).await.unwrap();
        assert_eq!(&server_buffer[..n], client_msg);

        let server_msg = b"Hello from server";
        service.send_tcp(&server_conn_id, server_msg).await.unwrap();
        
        let mut client_buffer = vec![0u8; 1024];
        let n = service.receive_tcp(&client_id, &mut client_buffer).await.unwrap();
        assert_eq!(&client_buffer[..n], server_msg);

        service.disconnect_tcp(&client_id).await.unwrap();
        service.disconnect_tcp(&server_conn_id).await.unwrap();
        service.close_tcp_server(&server_id).await.unwrap();
    }
}

#[cfg(test)]
mod log_service_tests {
    use super::*;

    #[tokio::test]
    async fn test_log_service_write() {
        let temp_dir = TempDir::new().unwrap();
        let service = LogService::new(temp_dir.path().to_path_buf());

        service.write_line("Test log entry").await.unwrap();

        let entries = tokio::fs::read_dir(temp_dir.path()).await.unwrap();
        let mut count = 0;
        let mut entries = entries;
        while entries.next_entry().await.unwrap().is_some() {
            count += 1;
        }
        assert!(count > 0);
    }

    #[tokio::test]
    async fn test_log_service_write_with_timestamp() {
        let temp_dir = TempDir::new().unwrap();
        let service = LogService::new(temp_dir.path().to_path_buf());

        service.write_with_timestamp("Test entry with timestamp").await.unwrap();

        let entries = tokio::fs::read_dir(temp_dir.path()).await.unwrap();
        let mut count = 0;
        let mut entries = entries;
        while entries.next_entry().await.unwrap().is_some() {
            count += 1;
        }
        assert!(count > 0);
    }

    #[tokio::test]
    async fn test_log_service_multiple_writes() {
        let temp_dir = TempDir::new().unwrap();
        let service = LogService::new(temp_dir.path().to_path_buf());

        for i in 0..10 {
            service.write_line(&format!("Log entry {}", i)).await.unwrap();
        }

        let entries = tokio::fs::read_dir(temp_dir.path()).await.unwrap();
        let mut count = 0;
        let mut entries = entries;
        while entries.next_entry().await.unwrap().is_some() {
            count += 1;
        }
        assert!(count > 0);
    }
}
