# Priority Flattening

## 职责

将配置优先级覆盖在编译期固化，数据面运行期不再判断覆盖关系。

## 输入

上一节点输出的编译上下文。

## 输出

更新后的编译上下文或明确失败原因。

## 不变量

- 不在数据面热路径做配置解析。
- 不让 ACL schema 类型泄漏到 runtime model。
- 不跳过 first-match 语义。
