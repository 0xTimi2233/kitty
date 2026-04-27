# Context Enrich

## 职责

按需补充高成本上下文。只有候选规则需要这些字段时才执行。

当前范围内，高成本上下文只包括 schema 已有的 process/user 类字段。

SNI 与 HTTP host 是 future scope，不属于当前 context enrich 必须支持的字段。

## 采集规则

- process/user 采集需要平台能力和权限。
- 只有候选规则需要 process/user 字段时才尝试采集。
- 采集失败时，依赖该字段的条件不匹配。
- 采集失败可记录按需 debug 事件。
- 采集失败不得导致无关规则失败。

## 输入

运行期请求上下文和当前 RuntimeModel handle。

## 输出

匹配结果、action 执行结果或可观测事件。

## 性能要求

- 避免无关上下文补充。
- 避免临时分配和字符串复制。
- 保持 first-match 语义。
