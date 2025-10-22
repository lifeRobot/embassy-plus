/// base cache
#[derive(Copy, Clone)]
pub struct BaseCache<const N: usize> {
    /// cache data
    bytes: [u8; N],
    /// data len
    len: usize,
}

/// custom method
impl<const N: usize> BaseCache<N> {
    /// create base cache
    #[inline]
    pub const fn new() -> BaseCache<N> {
        Self { bytes: [0; N], len: 0 }
    }

    /// 直接通过bytes生成数据
    #[inline]
    pub fn from_bytes(bytes: [u8; N]) -> Self {
        Self { bytes, len: N }
    }

    /// resetting the cache will be reset starting from 0 <br />
    /// if the bytes length is greater than N, false will be returned directly and the setting fails
    pub fn set(&mut self, bytes: &[u8]) -> bool {
        let len = bytes.len();
        if len > N { return false; }

        self.bytes[0..len].copy_from_slice(bytes);
        self.len = len;
        true
    }

    /// reset memory and remove invalid bytes to the left <br />
    /// returns whether the invalid cache was removed successfully
    pub fn reset(&mut self, invalid: usize) -> bool {
        // 长度为0直接退出，长度超过缓存长度，直接清空缓存并退出
        if invalid == 0 { return false; }
        if invalid >= self.len {
            self.clear();
            return true;
        }

        // 没有超过缓存，前移有效数据
        for i in 0..(self.len - invalid) {
            self.bytes[i] = self.bytes[i + invalid];
        }
        self.len -= invalid;
        true
    }

    /// append data to cache <br />
    /// if the appended length exceeds N, it will return directly
    pub fn push(&mut self, bytes: &[u8]) -> bool {
        let len = self.len + bytes.len();
        if len > N { return false; }

        self.bytes[self.len..len].copy_from_slice(bytes);
        self.len = len;
        true
    }

    /// add a byte to the cache <br />
    /// when the cache is full, the addition fails directly without resetting the cache
    pub fn put(&mut self, byte: u8) -> bool {
        if self.len >= N { return false; }
        self.bytes[self.len] = byte;
        self.len += 1;
        true
    }

    /// copy yourself directly <br />
    /// generally used to send data
    #[inline]
    pub fn copy_self(&mut self, other: &Self) {
        self.bytes.copy_from_slice(&other.bytes);
        self.len = other.len;
    }

    /// clean cache
    #[inline]
    pub fn clear(&mut self) {
        self.len = 0;
    }

    /// cache is empty
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// cache len
    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    /// cache is full
    #[inline]
    pub fn is_full(&self) -> bool {
        self.len == N
    }

    /// cache to bytes
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes[..self.len]
    }
}
