pub struct CircularBuffer {
    buffer: Vec<u8>,
    read_pos: usize,
    write_pos: usize,
    size: usize,
}

impl CircularBuffer {
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: vec![0u8; capacity],
            read_pos: 0,
            write_pos: 0,
            size: 0,
        }
    }

    pub fn write(&mut self, data: &[u8]) -> usize {
        let mut written = 0;
        for &byte in data {
            if self.size < self.buffer.len() {
                self.buffer[self.write_pos] = byte;
                self.write_pos = (self.write_pos + 1) % self.buffer.len();
                self.size += 1;
                written += 1;
            } else {
                break;
            }
        }
        written
    }

    pub fn read(&mut self, buffer: &mut [u8]) -> usize {
        let mut bytes_read = 0;
        let to_read = std::cmp::min(buffer.len(), self.size);

        for i in 0..to_read {
            buffer[i] = self.buffer[self.read_pos];
            self.read_pos = (self.read_pos + 1) % self.buffer.len();
            self.size -= 1;
            bytes_read += 1;
        }

        bytes_read
    }

    pub fn available(&self) -> usize {
        self.size
    }

    pub fn capacity(&self) -> usize {
        self.buffer.len()
    }

    pub fn clear(&mut self) {
        self.read_pos = 0;
        self.write_pos = 0;
        self.size = 0;
    }
}

impl Default for CircularBuffer {
    fn default() -> Self {
        Self::new(4096)
    }
}
