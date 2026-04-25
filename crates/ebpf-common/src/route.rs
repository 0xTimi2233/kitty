//! 路由标记 eBPF map 共享布局。

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RouteMarkValue {
    pub action: u8,
    pub outbound_id: u32,
}

pub const ROUTE_ACTION_DIRECT: u8 = 1;
pub const ROUTE_ACTION_REJECT: u8 = 2;
pub const ROUTE_ACTION_USERSPACE: u8 = 3;
