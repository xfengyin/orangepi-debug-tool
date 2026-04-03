//! Binary protocol for efficient frontend-backend communication

use bytes::{Buf, BufMut, Bytes, BytesMut};
use serde::{Deserialize, Serialize};

/// Binary message types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum MessageType {
    /// Serial data
    SerialData = 0x01,
    /// GPIO state change
    GpioState = 0x02,
    /// PWM state change
    PwmState = 0x03,
    /// Log message
    LogMessage = 0x04,
    /// Command request
    CommandRequest = 0x10,
    /// Command response
    CommandResponse = 0x11,
    /// Heartbeat
    Heartbeat = 0x20,
    /// Error
    Error = 0xFF,
}

impl MessageType {
    #[inline]
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0x01 => Some(MessageType::SerialData),
            0x02 => Some(MessageType::GpioState),
            0x03 => Some(MessageType::PwmState),
            0x04 => Some(MessageType::LogMessage),
            0x10 => Some(MessageType::CommandRequest),
            0x11 => Some(MessageType::CommandResponse),
            0x20 => Some(MessageType::Heartbeat),
            0xFF => Some(MessageType::Error),
            _ => None,
        }
    }
}

/// Binary message header
#[derive(Debug, Clone)]
pub struct MessageHeader {
    /// Magic number (0x4F50 = "OP")
    pub magic: u16,
    /// Message type
    pub msg_type: MessageType,
    /// Payload length
    pub length: u32,
    /// Sequence number
    pub sequence: u32,
    /// Timestamp
    pub timestamp: u64,
}

impl MessageHeader {
    pub const SIZE: usize = 18;
    pub const MAGIC: u16 = 0x4F50; // "OP"
    
    #[inline]
    pub fn new(msg_type: MessageType, length: u32, sequence: u32) -> Self {
        Self {
            magic: Self::MAGIC,
            msg_type,
            length,
            sequence,
            timestamp: Self::current_timestamp(),
        }
    }
    
    #[inline]
    pub fn encode(&self, buf: &mut BytesMut) {
        buf.put_u16(self.magic);
        buf.put_u8(self.msg_type as u8);
        buf.put_u32(self.length);
        buf.put_u32(self.sequence);
        buf.put_u64(self.timestamp);
    }
    
    #[inline]
    pub fn decode(buf: &mut BytesMut) -> Option<Self> {
        if buf.len() < Self::SIZE {
            return None;
        }
        
        let magic = buf.get_u16();
        if magic != Self::MAGIC {
            return None;
        }
        
        let msg_type = MessageType::from_u8(buf.get_u8())?;
        let length = buf.get_u32();
        let sequence = buf.get_u32();
        let timestamp = buf.get_u64();
        
        Some(Self {
            magic,
            msg_type,
            length,
            sequence,
            timestamp,
        })
    }
    
    #[inline]
    fn current_timestamp() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64
    }
}

/// Binary message codec
pub struct BinaryCodec;

impl BinaryCodec {
    #[inline]
    pub fn encode_serial_data(data: &[u8], sequence: u32) -> Bytes {
        let mut buf = BytesMut::with_capacity(MessageHeader::SIZE + data.len());
        let header = MessageHeader::new(MessageType::SerialData, data.len() as u32, sequence);
        header.encode(&mut buf);
        buf.extend_from_slice(data);
        buf.freeze()
    }
    
    #[inline]
    pub fn encode_gpio_state(pin: u32, value: u8, sequence: u32) -> Bytes {
        let mut buf = BytesMut::with_capacity(MessageHeader::SIZE + 5);
        let header = MessageHeader::new(MessageType::GpioState, 5, sequence);
        header.encode(&mut buf);
        buf.put_u32(pin);
        buf.put_u8(value);
        buf.freeze()
    }
    
    #[inline]
    pub fn encode_pwm_state(channel: u32, duty_cycle: f32, sequence: u32) -> Bytes {
        let mut buf = BytesMut::with_capacity(MessageHeader::SIZE + 8);
        let header = MessageHeader::new(MessageType::PwmState, 8, sequence);
        header.encode(&mut buf);
        buf.put_u32(channel);
        buf.put_f32(duty_cycle);
        buf.freeze()
    }
}