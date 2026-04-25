# Normalize 规则

schema 层做轻量输入 normalize。

- 一般 tag：`trim + lowercase`。
- 一般枚举：由 serde rename 处理。
- path：只 trim。
- domain / suffix / keyword：`trim + lowercase`。
- domain suffix：额外去掉前导点。
- domain regex：只 trim。
- 单个值和数组输入统一转为 `Vec<T>`。

match-pipeline 入口也需要对运行时输入做同等 normalize，保证配置侧和流量侧一致。
