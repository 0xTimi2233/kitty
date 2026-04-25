//! DNS eBPF map 共享布局。

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DnsCacheKey {
    pub domain_hash_hi: u64,
    pub domain_hash_lo: u64,
    pub qtype: u16,
    pub qclass: u16,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DnsCacheValueV4 {
    pub action: u8,
    pub ttl: u32,
    pub ipv4: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DnsCacheValueV6 {
    pub action: u8,
    pub ttl: u32,
    pub ipv6: [u8; 16],
}
