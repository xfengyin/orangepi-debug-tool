use std::sync::Arc;
use std::time::Instant;
use test_lib::adapters::MockSerialAdapter;
use test_lib::adapters::serial::CircularBuffer;
use test_lib::services::SerialService;
use test_lib::adapters::SerialConfig;

#[tokio::test]
async fn test_high_throughput_serial() {
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

    let start = Instant::now();
    let mut total_bytes = 0u64;
    let iterations = 1000;

    for _ in 0..iterations {
        let data: Vec<u8> = (0..1024).map(|i| (i % 256) as u8).collect();
        service.send(&id, &data).await.unwrap();
        total_bytes += data.len() as u64;
    }

    let elapsed = start.elapsed();
    let throughput = (total_bytes as f64 * 8.0) / elapsed.as_secs_f64() / 1000.0;

    println!("Serial throughput: {:.2} kbps ({} bytes in {:?})", throughput, total_bytes, elapsed);
    
    assert!(elapsed.as_secs() < 10, "Should complete within 10 seconds");

    service.disconnect(&id).await.unwrap();
}

#[tokio::test]
async fn test_circular_buffer_performance() {
    let mut buffer = CircularBuffer::new(8192);

    let start = Instant::now();
    let iterations = 100_000;

    for i in 0..iterations {
        let data = vec![(i % 256) as u8];
        buffer.write(&data);
    }

    let elapsed = start.elapsed();
    println!("CircularBuffer write: {} ops in {:?}", iterations, elapsed);
    assert!(elapsed.as_millis() < 100, "Should complete within 100ms");
}

#[test]
fn test_circular_buffer_wrap_around() {
    let mut buffer = CircularBuffer::new(10);

    buffer.write(&[1, 2, 3, 4, 5]);
    assert_eq!(buffer.available(), 5);

    let mut read_buf = [0u8; 3];
    let n = buffer.read(&mut read_buf);
    assert_eq!(n, 3);
    assert_eq!(buffer.available(), 2);

    buffer.write(&[6, 7, 8, 9, 10, 11]);
    assert_eq!(buffer.available(), 10);

    let mut read_buf = [0u8; 10];
    let n = buffer.read(&mut read_buf);
    assert_eq!(n, 10);
    assert_eq!(buffer.available(), 0);
}

#[tokio::test]
async fn test_multiple_connection_performance() {
    let mock = Arc::new(MockSerialAdapter::new());
    let service = SerialService::new(mock);

    let start = Instant::now();
    let num_connections = 50;

    let mut connection_ids = vec![];
    
    for i in 0..num_connections {
        let config = SerialConfig {
            port_name: format!("/dev/ttyUSB{}", i % 3),
            baudrate: 115200,
            data_bits: 8,
            stop_bits: 1,
            parity: "none".to_string(),
        };
        let id = service.connect(format!("/dev/ttyUSB{}", i % 3), config).await.unwrap();
        connection_ids.push(id);
    }

    for id in &connection_ids {
        let data = vec![1, 2, 3, 4];
        service.send(id, &data).await.unwrap();
    }

    for id in &connection_ids {
        let mut buffer = vec![0u8; 4];
        service.read(id, &mut buffer).await.unwrap();
    }

    for id in &connection_ids {
        service.disconnect(id).await.unwrap();
    }

    let elapsed = start.elapsed();
    println!("Multiple connections test: {} connections in {:?}", num_connections, elapsed);
    assert!(elapsed.as_secs() < 5, "Should complete within 5 seconds");
}

#[test]
fn test_circular_buffer_overflow_handling() {
    let mut buffer = CircularBuffer::new(5);

    let written = buffer.write(&[1, 2, 3, 4, 5, 6, 7]);
    assert_eq!(written, 5);
    assert_eq!(buffer.available(), 5);

    let mut read_buf = [0u8; 5];
    let n = buffer.read(&mut read_buf);
    assert_eq!(n, 5);
    assert_eq!(buffer.available(), 0);

    let written = buffer.write(&[8, 9]);
    assert_eq!(written, 2);
    assert_eq!(buffer.available(), 2);
}

#[test]
fn test_circular_buffer_clear() {
    let mut buffer = CircularBuffer::new(10);

    buffer.write(&[1, 2, 3, 4, 5]);
    assert_eq!(buffer.available(), 5);

    buffer.clear();
    assert_eq!(buffer.available(), 0);

    let written = buffer.write(&[6, 7, 8]);
    assert_eq!(written, 3);
    assert_eq!(buffer.available(), 3);

    let mut read_buf = [0u8; 3];
    let n = buffer.read(&mut read_buf);
    assert_eq!(n, 3);
    assert_eq!(read_buf, [6, 7, 8]);
}
