//! eBPF DNS cache key/value。

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DnsCacheKey {
    pub domain_hash_hi: u64,
    pub domain_hash_lo: u64,
    pub qtype: u16,
    pub qclass: u16,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DnsCacheValue {
    pub action: u8,
    pub record_count: u8,
    pub ttl: u32,
}
