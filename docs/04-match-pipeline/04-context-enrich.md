# Context Enrich

## 职责

按需补充 process、user、SNI、HTTP host 等高成本上下文。只有候选规则需要这些字段时才执行。

## 输入

运行期请求上下文和当前 RuntimeModel handle。

## 输出

匹配结果、action 执行结果或可观测事件。

## 性能要求

- 避免无关上下文补充。
- 避免临时分配和字符串复制。
- 保持 first-match 语义。
