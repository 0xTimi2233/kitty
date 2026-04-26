# Ingress Classify

## 职责

识别流量入口类型：DNS UDP、DNS TCP、proxy session、eBPF TC ingress 或其他已支持入口。

## 输入

运行期请求上下文和当前 RuntimeModel handle。

## 输出

匹配结果、action 执行结果或可观测事件。

## 性能要求

- 避免无关上下文补充。
- 避免临时分配和字符串复制。
- 保持 first-match 语义。
