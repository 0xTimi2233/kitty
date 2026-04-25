# 01 Ingress Classify

## 职责

Ingress Classify 识别流量入口、协议类型和可用 fast path，为后续匹配构建最小请求对象。

## 输入

- inbound tag。
- socket metadata。
- packet metadata。
- 当前 Runtime Model。

## 输出

- `MatchRequest`
- `IngressKind`

## 分类结果

- DNS UDP/53。
- DNS TCP/53。
- Proxy TCP。
- Proxy UDP。
- eBPF TC ingress。
- eBPF XDP DNS cache hit。

## eBPF DNS 策略

- UDP/53 cache hit：kernel-side 直接响应。
- UDP/53 cache miss：进入用户态 DNS inbound。
- TCP/53：进入用户态 DNS inbound。
- A / AAAA 以外的 DNS 查询进入用户态。

## 约束

- 该阶段不执行规则匹配。
- 该阶段不补充 process/user 信息。
- 该阶段只构建最小上下文。

## 测试要点

- UDP/53 miss 能进入用户态 DNS pipeline。
- TCP/53 不走 DNS eBPF fast response。
- 非 DNS 流量进入 route match pipeline。
