# 02 Compile Pipeline Tests

## 目标

验证 Compile Pipeline 能从 schema 配置生成正确的 Runtime Plan，并在各阶段保持明确的输入输出边界。

## 测试范围

### Normalize

- action 快捷写法转换为对象级 action。
- DefaultRule 和 LogicalRule 转换为统一 Rule IR。
- domain_suffix 去前导点。

### Basic validate

- tag 唯一性。
- remote rule_set URL 协议。
- rule_set update interval 下限。

### Semantic validate

- DNS server 引用解析。
- outbound 引用解析。
- route final 解析。
- 默认 Direct 编译期固化。

### Loop check

- DNS resolver cycle。
- detour cycle。
- outbound dependency cycle。

### Rule set

- 未引用 rule_set 不下载、不读取。
- remote cache 过期时 start 同步刷新。
- local rule_set meta 写入。
- KSR header verify。

### Expand merge

- rule_set 分支与外层 rule 按 group 合并。
- 不产生笛卡尔积。
- 多分支保持 OR。

### Index compile

- exact domain index。
- suffix / keyword / regex index。
- CIDR index。
- port index。
- rule order table。

## 测试方法

每个阶段应提供小型 fixture，验证阶段输出结构，而不是只测最终结果。

## 回归要求

任何修改规则语义、index 编译或 action 解析的代码，都必须更新对应 compile pipeline 测试。
