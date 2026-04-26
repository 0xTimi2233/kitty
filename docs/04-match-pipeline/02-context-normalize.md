# Context Normalize

## 职责

对运行期输入做轻量 normalize，例如 domain trim/lowercase、suffix 统一和 qtype 标准化。

## 输入

运行期请求上下文和当前 RuntimeModel handle。

## 输出

匹配结果、action 执行结果或可观测事件。

## 性能要求

- 避免无关上下文补充。
- 避免临时分配和字符串复制。
- 保持 first-match 语义。
