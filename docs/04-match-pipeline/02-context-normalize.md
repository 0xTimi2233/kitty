# 02 Context Normalize

## 职责

Context Normalize 对运行期输入做与 schema 侧一致的 normalize，确保匹配行为稳定。

## 输入

- `MatchRequest`

## 输出

- `NormalizedMatchContext`

## 规则

- host/domain：`trim + lowercase`。
- domain_suffix 查询辅助值：去掉前导点。
- tag：`trim + lowercase`。
- path：只执行 `trim`。
- IP 地址：解析为二进制地址。
- port：使用整数表示。

## 设计原则

- 尽量原地处理字符串，避免无意义分配。
- 已 normalize 的字段不得重复 normalize。
- 正则输入不做 lowercase。

## 非职责

- 不执行 DNS cache 查询。
- 不补充 process/user。
- 不执行规则匹配。

## 测试要点

- 大小写域名能命中相同规则。
- 前后空白不影响匹配。
- path 不会被 lowercase。
- regex 输入保持原样。
