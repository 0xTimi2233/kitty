# DNS Cache Lookup

## 职责

DNS 请求先查用户态 DNS cache；eBPF cache hit 已在更早路径处理。cache miss 继续进入 matcher。

## 输入

运行期请求上下文和当前 RuntimeModel handle。

## 输出

匹配结果、action 执行结果或可观测事件。

## 性能要求

- 避免无关上下文补充。
- 避免临时分配和字符串复制。
- 保持 first-match 语义。
