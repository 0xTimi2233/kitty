# eBPF 分层

Kitty 使用 Aya 路线。

分层：

- `ebpf-common`：用户态和内核态共享的 `repr(C)` 布局。
- `ebpf-programs`：Aya eBPF program 骨架。
- `infrastructure::ebpf`：用户态 loader、attach、map sync、cleanup。

DNS fast path：

- UDP/53 cache hit：eBPF 直接响应。
- UDP/53 cache miss：进入用户态 DNS inbound。
- TCP/53：进入用户态 DNS inbound。
- eBPF 只存储 A / AAAA 结果。

Route mark：

- DIRECT。
- REJECT。
- outbound id。

map 生命周期由用户态管理。
