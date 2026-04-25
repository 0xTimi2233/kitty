# 05 eBPF Tests

## 目标

验证 userspace 与 kernel-side eBPF 程序之间的数据 layout、map key/value 和同步语义。

## 测试范围

### Shared layout

- `#[repr(C)]` key/value 布局稳定。
- Rust userspace 与 eBPF program 看到的字段大小一致。
- IPv4 / IPv6 key 不混用。

### DNS map

- DNS key 由 normalized domain hash、qtype、qclass 构成。
- 只写入 A / AAAA。
- qclass 非 IN 不写入。
- hash 碰撞时回退用户态。

### Route mark map

- Direct mark 正确写入。
- Reject mark 正确写入。
- Outbound mark 正确写入。

### Lifecycle

- Runtime 切换后旧 generation entry 能清理。
- TTL 过期 entry 能清理。
- map 写入失败能记录错误事件。

## 测试层级

### Unit tests

验证 key/value 构造和 layout。

### Integration tests

在支持 eBPF 的测试环境中加载程序，验证 map 写入和读取。

### Fallback tests

在不支持 eBPF 的环境中，测试 userspace 能明确进入不可用错误路径。

## 回归要求

修改 eBPF key/value、map sync 或 Runtime generation 策略时必须运行本组测试。
