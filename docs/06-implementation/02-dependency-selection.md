# 三方依赖选型

当前代码只引入 ACL schema 必需依赖。其他依赖在实现对应模块时再按需加入。

选型方向：

- async runtime/io：tokio。
- CPU 并行：rayon。
- shutdown：CancellationToken。
- duration serde：humantime-serde。
- bitmap：roaring。
- keyword：aho-corasick。
- regex：regex-automata lazy DFA。
- DNS message codec：hickory-proto。
- cache：moka。
- logging：tracing 系列。
- error：thiserror。
- 小集合优化：arrayvec、smallvec。
- pid lock：fs2。
- eBPF：Aya。

依赖进入 Cargo.toml 的原则：只有当前 crate 实际使用时才引入。
