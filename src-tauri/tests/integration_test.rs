use std::sync::Arc;
use test_lib::adapters::MockSerialAdapter;
use test_lib::adapters::TokioNetworkAdapter;
use test_lib::services::{SerialService, NetworkService};
use test_lib::adapters::SerialConfig;

#[tokio::test]
async fn test_full_serial_workflow() {
    let mock = Arc::new(MockSerialAdapter::new());
    let service = SerialService::new(mock);

    let ports = service.list_ports().await.unwrap();
    assert!(!ports.is_empty());

    let config = SerialConfig {
        port_name: ports[0].name.clone(),
        baudrate: 115200,
        data_bits: 8,
        stop_bits: 1,
        parity: "none".to_string(),
    };
    let id = service.connect(ports[0].name.clone(), config).await.unwrap();

    let data = vec![0x01, 0x02, 0x03, 0x04];
    service.send(&id, &data).await.unwrap();

    let mut buffer = vec![0u8; 1024];
    let n = service.read(&id, &mut buffer).await.unwrap();
    assert_eq!(&buffer[..n], &data);

    let stats = service.get_stats(&id).await.unwrap();
    assert_eq!(stats.bytes_sent, 4);
    assert_eq!(stats.bytes_received, 4);

    service.disconnect(&id).await.unwrap();
}

#[tokio::test]
async fn test_full_tcp_workflow() {
    let adapter = Arc::new(TokioNetworkAdapter::new());
    let service = NetworkService::new(adapter);

    let server_id = service.create_tcp_server(19993, 10).await.unwrap();

    let client_id = service.connect_tcp("127.0.0.1", 19993).await.unwrap();
    let server_conn_id = service.accept_tcp_connection(&server_id).await.unwrap();

    let test_data = b"Integration test data";
    service.send_tcp(&client_id, test_data).await.unwrap();

    let mut buffer = vec![0u8; 1024];
    let n = service.receive_tcp(&server_conn_id, &mut buffer).await.unwrap();
    assert_eq!(&buffer[..n], test_data);

    let response = b"Response";
    service.send_tcp(&server_conn_id, response).await.unwrap();

    let mut recv_buffer = vec![0u8; 1024];
    let n = service.receive_tcp(&client_id, &mut recv_buffer).await.unwrap();
    assert_eq!(&recv_buffer[..n], response);

    service.disconnect_tcp(&client_id).await.unwrap();
    service.disconnect_tcp(&server_conn_id).await.unwrap();
    service.close_tcp_server(&server_id).await.unwrap();
}

#[tokio::test]
async fn test_concurrent_serial_connections() {
    let mock = Arc::new(MockSerialAdapter::new());
    let service = SerialService::new(mock);

    let mut handles = vec![];
    
    for i in 0..3 {
        let service = service.clone();
        let handle = tokio::spawn(async move {
            let config = SerialConfig {
                port_name: format!("/dev/ttyUSB{}", i),
                baudrate: 115200,
                data_bits: 8,
                stop_bits: 1,
                parity: "none".to_string(),
            };
            let id = service.connect(format!("/dev/ttyUSB{}", i), config).await.unwrap();
            
            let data = vec![i as u8; 4];
            service.send(&id, &data).await.unwrap();
            
            let mut buffer = vec![0u8; 4];
            let _n = service.read(&id, &mut buffer).await.unwrap();
            
            service.disconnect(&id).await.unwrap();
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }
}

#[tokio::test]
async fn test_concurrent_tcp_connections() {
    let adapter = Arc::new(TokioNetworkAdapter::new());
    let service = NetworkService::new(adapter);

    let server_id = service.create_tcp_server(19992, 10).await.unwrap();

    let mut handles = vec![];
    for i in 0..5 {
        let service = service.clone();
        let handle = tokio::spawn(async move {
            let id = service.connect_tcp("127.0.0.1", 19992).await.unwrap();
            let data = format!("Connection {}", i);
            service.send_tcp(&id, data.as_bytes()).await.unwrap();
            service.disconnect_tcp(&id).await.unwrap();
        });
        handles.push(handle);
    }

    for _ in 0..5 {
        let _conn_id = service.accept_tcp_connection(&server_id).await.unwrap();
    }

    for handle in handles {
        handle.await.unwrap();
    }

    service.close_tcp_server(&server_id).await.unwrap();
}

#[tokio::test]
async fn test_mixed_protocol_communication() {
    let serial_mock = Arc::new(MockSerialAdapter::new());
    let serial_service = SerialService::new(serial_mock);
    
    let network_adapter = Arc::new(TokioNetworkAdapter::new());
    let network_service = NetworkService::new(network_adapter);

    let serial_config = SerialConfig {
        port_name: "/dev/ttyUSB0".to_string(),
        baudrate: 115200,
        data_bits: 8,
        stop_bits: 1,
        parity: "none".to_string(),
    };
    let serial_id = serial_service.connect("/dev/ttyUSB0".to_string(), serial_config).await.unwrap();

    let server_id = network_service.create_tcp_server(19988, 10).await.unwrap();

    let serial_data = vec![0xDE, 0xAD, 0xBE, 0xEF];
    serial_service.send(&serial_id, &serial_data).await.unwrap();
    
    let mut recv_buffer = vec![0u8; 4];
    let n = serial_service.read(&serial_id, &mut recv_buffer).await.unwrap();
    assert_eq!(n, 4);

    let client_id = network_service.connect_tcp("127.0.0.1", 19988).await.unwrap();
    let server_conn_id = network_service.accept_tcp_connection(&server_id).await.unwrap();

    let network_data = b"Network data";
    network_service.send_tcp(&client_id, network_data).await.unwrap();
    
    let mut net_buffer = vec![0u8; 1024];
    let n = network_service.receive_tcp(&server_conn_id, &mut net_buffer).await.unwrap();
    assert_eq!(&net_buffer[..n], network_data);

    serial_service.disconnect(&serial_id).await.unwrap();
    network_service.disconnect_tcp(&client_id).await.unwrap();
    network_service.disconnect_tcp(&server_conn_id).await.unwrap();
    network_service.close_tcp_server(&server_id).await.unwrap();
}

#[tokio::test]
async fn test_service_stats_tracking() {
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

    for i in 0..10 {
        let data: Vec<u8> = (0..100).map(|j| (i * 100 + j) as u8).collect();
        service.send(&id, &data).await.unwrap();
        
        let mut buffer = vec![0u8; 100];
        service.read(&id, &mut buffer).await.unwrap();
    }

    let stats = service.get_stats(&id).await.unwrap();
    assert_eq!(stats.bytes_sent, 1000);
    assert_eq!(stats.packets_sent, 10);
    assert_eq!(stats.bytes_received, 1000);
    assert_eq!(stats.packets_received, 10);
    assert_eq!(stats.errors, 0);

    service.disconnect(&id).await.unwrap();
}
