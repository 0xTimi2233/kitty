# Config Decode

## 职责

读取配置文件并使用 ACL schema 反序列化为 SchemaConfig。此阶段完成 decode-time local normalize 和默认值填充。必填字段缺失由反序列化阶段直接失败。

## 输入

上一节点输出的编译上下文。

## 输出

更新后的编译上下文或明确失败原因。

## 不变量

- 不在数据面热路径做配置解析。
- 不让 ACL schema 类型泄漏到 runtime model。
- 不跳过 first-match 语义。
