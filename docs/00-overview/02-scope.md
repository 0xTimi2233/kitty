# 范围

当前范围：

- DNS server 类型：tcp、udp、tls、quic、https、h3。
- inbound 类型：direct、socks、vless、dns、tc。
- outbound 类型：direct、socks、vless。
- route 与 dns 规则支持 leaf rule、logical rule、rule_set 引用。
- rule_set 支持 local、remote；format 支持 source、binary；binary 后缀为 `.ksr`。
- eBPF 路线使用 Aya，第一阶段只建立架构和数据布局骨架。

当前不在范围内：

- 多平台适配。
- 内核态完整 DNS upstream 转发。
- 运行态动态修改 matcher 结构。
- 长期历史数据平台。
