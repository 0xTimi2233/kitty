#![no_std]

//! Aya eBPF program 骨架。
//!
//! 这里保留内核态程序模块边界；具体 XDP/TC program 在实现阶段补充。

pub mod tc_route_mark;
pub mod xdp_dns_cache;
