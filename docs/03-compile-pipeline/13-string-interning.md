# 13 String Interning

## 职责

String Interning 将重复出现的 tag、domain、path、action name 等字符串驻留为稳定 ID 或共享字符串，减少运行态内存占用和比较成本。

## 输入

- `PriorityFlattenedRules`
- resolved server / outbound / inbound 配置。

## 输出

- `InternTable`
- `InternedConfig`

## 驻留对象

- inbound tag。
- outbound tag。
- DNS server tag。
- rule_set tag。
- domain exact。
- domain suffix。
- domain keyword。
- process name。
- process path。
- user name。
- interface name。

## 设计原则

- 编译期可以使用可变 map 构造驻留表。
- Runtime Model 只读取稳定 ID 或共享字符串。
- 热路径避免重复分配和重复 lowercase。
- 需要对外展示的 tag 保留反查表。

## 输出约束

- 运行态不得依赖临时字符串生命周期。
- 所有 ID 必须在一次 Runtime generation 内稳定。
- 不同 Runtime generation 之间不要求 ID 保持一致。

## 测试要点

- 重复字符串只驻留一次。
- 反查 tag 能返回 normalize 后字符串。
- Runtime generation 切换不复用旧 generation 的无效引用。
