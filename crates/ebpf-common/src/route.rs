//! eBPF route mark key/value。

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RouteMarkValue {
    pub action: u8,
    pub outbound_id: u32,
}
