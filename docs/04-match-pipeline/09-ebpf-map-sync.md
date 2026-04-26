# eBPF Map Sync

## 职责

DNS 结果和 route action 完成后，由用户态写入 eBPF map。map 生命周期、清理和写入均在用户态控制。

## 输入

运行期请求上下文和当前 RuntimeModel handle。

## 输出

匹配结果、action 执行结果或可观测事件。

## 性能要求

- 避免无关上下文补充。
- 避免临时分配和字符串复制。
- 保持 first-match 语义。
