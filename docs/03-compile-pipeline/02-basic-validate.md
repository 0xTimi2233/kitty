# Basic Validate

## 职责

只检查单字段或单对象局部合法性，例如空字符串、URL scheme、duration 下限、CIDR 格式、正则可编译性。不得检查 tag 重复或跨对象引用。

## 输入

上一节点输出的编译上下文。

## 输出

更新后的编译上下文或明确失败原因。

## 不变量

- 不在数据面热路径做配置解析。
- 不让 ACL schema 类型泄漏到 runtime model。
- 不跳过 first-match 语义。
