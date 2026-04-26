# String Interning

## 职责

对 domain、tag、rule metadata 等高频字符串进行驻留，降低 runtime 内存占用和比较成本。

## 输入

上一节点输出的编译上下文。

## 输出

更新后的编译上下文或明确失败原因。

## 不变量

- 不在数据面热路径做配置解析。
- 不让 ACL schema 类型泄漏到 runtime model。
- 不跳过 first-match 语义。
