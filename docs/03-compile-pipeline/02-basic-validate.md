# Basic Validate

## 职责

Basic Validate 只检查单字段或单对象局部合法性，不检查 tag 重复、不解析跨对象引用、不构建 Rule IR。

本阶段必须覆盖：

- required 字段缺失或空字符串。
- `dns.servers` 为空。
- `inbounds` 为空。
- `outbounds` 为空。
- `listen_port` 未显式配置、为 0 或超出合法端口范围。
- URL scheme 不合法。
- duration 非法或低于产品下限。
- remote `rule_set.update_interval` 非 0 且小于 15 分钟。
- CIDR 格式不合法。
- 正则不可编译。
- path 字段 trim 后为空。

`dns.final`、`route.final`、action target、detour 和 resolver 引用存在性由 semantic validate 处理。

## 输入

上一节点输出的编译上下文。

## 输出

更新后的编译上下文或明确失败原因。

失败原因必须可定位到配置路径。错误消息使用英文。

## 不变量

- 不在数据面热路径做配置解析。
- 不让 ACL schema 类型泄漏到 runtime model。
- 不跳过 first-match 语义。
- 不做跨对象引用解析。
