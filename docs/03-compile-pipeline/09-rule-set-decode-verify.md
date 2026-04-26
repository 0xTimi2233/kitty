# Rule Set Decode Verify

## 职责

解码 source rule_set 或校验 KSR 文件头。KSR 深层语义由生成流程保证。

## 输入

上一节点输出的编译上下文。

## 输出

更新后的编译上下文或明确失败原因。

## 不变量

- 不在数据面热路径做配置解析。
- 不让 ACL schema 类型泄漏到 runtime model。
- 不跳过 first-match 语义。
