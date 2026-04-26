# Logging 与 Error

日志使用统一事件模型：

```text
timestamp level event msg fields
```

要求：

- event 是稳定枚举值。
- msg 使用 Display 文本。
- fields 平铺输出。
- 反序列化错误将带路径的 display 文本放入 msg。
- 错误消息和日志消息使用英文。
- 注释和编排说明使用中文。
