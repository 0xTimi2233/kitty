# 依赖选型

当前骨架只在 Cargo 中引入 schema 所需依赖。

已进入骨架依赖：

- `serde`
- `serde_with`
- `humantime-serde`
- `humantime`
- `macaddr`

后续实现阶段候选：

- async runtime/io：`tokio`
- CPU 并发：`rayon`
- shutdown：`tokio-util`
- bitmap：`roaring`
- keyword：`aho-corasick`
- regex：`regex-automata`
- DNS protocol codec：`hickory-proto`
- cache：`moka`
- logging：`tracing`, `tracing-subscriber`, `tracing-appender`
- error：`thiserror`
- small allocation：`arrayvec`, `smallvec`
- pid lock：`fs2`
- signal：`nix`
- eBPF：`aya`
- hash map：`rustc-hash`
- file hash：`blake3`
- CIDR：`ip_network`、`ip_network_table`

DNS 上游 transport 需要在实现阶段单独 PoC。协议编解码优先保持在 infrastructure 防腐层内。
