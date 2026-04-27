# 范围

本文件描述当前 `docs/` 需求范围。这里的范围用于后续完善文档和正式 planning 输入，不表示需求永久冻结。

## 当前范围

- DNS server 类型：tcp、udp、tls、quic、https、h3。
- inbound 类型：direct、socks、vless、dns、tc。
- outbound 类型：direct、socks、vless。
- route 与 dns 规则支持 leaf rule、logical rule、rule_set 引用。
- rule_set 支持 local、remote；format 支持 source、binary；binary 后缀为 `.ksr`。
- eBPF 路线使用 Aya，eBPF 是当前必需能力；权限不足、load 失败或 attach 失败会导致 start/reload 失败。
- privileged port bind 失败会导致对应 start/reload 失败。
- matcher 当前支持 schema 已定义的低成本字段，以及 process/user 类可选高成本上下文。

## Future scope

- API 管理面。当前管理面只要求 CLI 和 signal；API 只作为未来能力，不进入当前需求细节。
- SNI 与 HTTP host 匹配上下文。
- 更完整的 KSR version compatibility 策略。
- compile/match pipeline 全节点输入、输出和错误矩阵细化。

## 当前不在范围内

- 多平台适配。
- 内核态完整 DNS upstream 转发。
- 运行态动态修改 matcher 结构。
- 长期历史数据平台。
