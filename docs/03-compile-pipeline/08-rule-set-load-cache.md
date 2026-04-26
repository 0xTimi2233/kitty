# Rule Set Load Cache

## 职责

加载 local rule_set，检查 remote rule_set meta 与缓存。启动阶段过期缓存必须同步刷新；无缓存且下载失败时失败。

## 输入

上一节点输出的编译上下文。

## 输出

更新后的编译上下文或明确失败原因。

## 不变量

- 不在数据面热路径做配置解析。
- 不让 ACL schema 类型泄漏到 runtime model。
- 不跳过 first-match 语义。
