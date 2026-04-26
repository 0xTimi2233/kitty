# Rule Set Reference Prune

## 职责

只保留当前 runtime generation 实际引用的 rule_set。未引用 remote rule_set 不下载、不解码、不生成缓存产物。

## 输入

上一节点输出的编译上下文。

## 输出

更新后的编译上下文或明确失败原因。

## 不变量

- 不在数据面热路径做配置解析。
- 不让 ACL schema 类型泄漏到 runtime model。
- 不跳过 first-match 语义。
