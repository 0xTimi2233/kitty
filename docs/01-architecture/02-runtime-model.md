# Runtime Model

runtime model 是 compile-pipeline 产出的只读模型。

要求：

- 使用稳定 ID 代替运行期字符串查找。
- 默认值、优先级覆盖和引用解析在编译期完成。
- 数据面不做 schema 级判断。
- runtime 发布采用原子替换。
- 旧 runtime 在无引用后释放。

建议模型：

- `DnsRuntime`
- `RouteRuntime`
- `OutboundRuntime`
- `InboundRuntimeSpec`
- `MatcherRuntime`
- `EbpfRuntimePlan`
