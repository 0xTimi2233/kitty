# eBPF 架构

eBPF 使用 Aya 路线，是当前范围内的必需能力，不是 optional fallback。

`ebpf-programs` 是 kernel-side eBPF program，受 no_std、verifier、栈大小和 helper 限制。

`ebpf-common` 保存用户态和 eBPF program 共享的 repr(C) key/value。

`infrastructure::ebpf` 是 userspace loader，负责 load、attach、map update、map cleanup 和 lifecycle。

## 失败策略

- eBPF load 失败时，start/reload 失败。
- eBPF attach 失败时，start/reload 失败。
- eBPF 权限不足时，start/reload 失败。
- privileged port bind 失败时，对应 start/reload 失败。

external reload 失败不得替换旧 runtime。

## 第一阶段

- UDP/53 cache hit 在 eBPF 快速响应。
- UDP/53 cache miss 进入用户态 DNS inbound。
- TCP/53 进入用户态 DNS inbound。
- 仅 A/AAAA 记录进入 eBPF DNS cache。
