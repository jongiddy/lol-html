pub struct Buffer {
    bytes: Box<[u8]>,
    capacity: usize,
    watermark: usize,
}

impl Buffer {
    pub fn new(capacity: usize) -> Self {
        Buffer {
            bytes: vec![0; capacity].into(),
            capacity,
            watermark: 0,
        }
    }

    #[inline]
    pub fn write(&mut self, chunk: Vec<u8>) -> Result<(), &'static str> {
        let chunk_len = chunk.len();

        if self.watermark + chunk_len <= self.capacity {
            let new_watermark = self.watermark + chunk_len;

            (&mut self.bytes[self.watermark..new_watermark]).copy_from_slice(&chunk);
            self.watermark = new_watermark;

            Ok(())
        } else {
            Err("Buffer capacity exceeded")
        }
    }

    #[inline]
    pub fn peek_at(&self, pos: usize) -> Option<u8> {
        if pos < self.watermark {
            Some(self.bytes[pos])
        } else {
            None
        }
    }

    #[inline]
    pub fn slice(&self, start: usize, end: usize) -> &[u8] {
        &self.bytes[start..end]
    }
}