# eBPF Tests

覆盖：

- `ebpf-common` repr(C) 布局。
- map key/value。
- UDP/53 cache hit。
- UDP/53 cache miss 回用户态。
- TCP/53 回用户态。
- eBPF load 失败导致 start/reload 失败。
- eBPF attach 失败导致 start/reload 失败。
- eBPF 权限不足导致 start/reload 失败。
- privileged port bind 失败导致对应 start/reload 失败。
- external reload 中 eBPF 失败不得替换旧 runtime。

eBPF 是当前范围内的必需能力，不测试 optional fallback。
