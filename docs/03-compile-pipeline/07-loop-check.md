# 07 Loop Check

## 职责

Loop Check 检查 DNS、outbound、detour、resolver、rule_set fetch 等路径中的依赖回环。

## 输入

- `SemanticValidConfig`
- `ResolvedReferenceSet`

## 输出

- `LoopCheckedConfig`
- `DependencyGraph`

## 检查对象

### DNS resolver 依赖

检查 DNS server 的 resolver 相关引用，避免形成无法解析自身上游地址的循环。

### detour 依赖

检查 DNS server detour、outbound detour、route final 和 route action 之间的间接依赖。

### rule_set fetch 依赖

remote rule_set 下载需要解析 URL host。启动阶段必须使用已解析的 bootstrap path，不得依赖尚未展开的 rule_set 规则。

### outbound 依赖

检查 outbound group、detour、server address resolver 之间是否存在循环。

## 原则

- 回环检测必须在 rule_set 下载前完成可检测部分。
- 无法在静态阶段完全确认的路径，必须降级为明确的启动错误或 runtime degraded 状态，不得静默进入不可用状态。
- 依赖图应保留到 Runtime Metadata，用于诊断和拓扑导出。

## 错误输出

错误需要包含：

- cycle path。
- 参与回环的对象 tag。
- 触发配置路径。

## 测试要点

- DNS server 自引用 resolver 会失败。
- DNS server detour 经 outbound 回到自身会失败。
- remote rule_set fetch 不依赖未展开 rule_set。
- 多节点 cycle 能输出完整路径。
