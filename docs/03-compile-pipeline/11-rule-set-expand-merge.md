# 11 Rule Set Expand Merge

## 职责

Rule Set Expand Merge 将 rule_set 引用展开，并将 rule_set 分支条件合并到外层规则条件中。

## 输入

- 主配置规则 IR。
- `DecodedRuleSet`。
- `SymbolTable`。

## 输出

- `ExpandedRuleList`

## Rule IR

规则统一转换为以下节点：

- `and`
- `or`
- `not`
- `leaf`

Default rule 和 logical rule 不作为最终 IR 节点类型保留。

## 条件组

条件按照 group 合并：

| Group | 示例字段 | 组内语义 |
| --- | --- | --- |
| DomainGroup | domain / domain_suffix / domain_keyword / domain_regex | OR |
| DestIpGroup | ip_cidr / ip_is_private | OR |
| DestPortGroup | port / port_range | OR |
| SourceIpGroup | source_ip_cidr / source_ip_is_private | OR |
| SourcePortGroup | source_port / source_port_range | OR |
| ProcessGroup | process_name / process_path | OR |
| UserGroup | user / user_id | OR |

不同 group 之间是 AND。

## rule_set 合并语义

外层 rule 与 rule_set 分支合并时，不做笛卡尔积展开，而是按 group 直接合并。

示例：

外层 rule：

```text
domain = ["www.example.com"]
port = [443]
```

rule_set 分支：

```text
domain = ["news.example.com", "ad.example.com"]
domain_suffix = ["cdn.example.com"]
```

合并结果：

```text
DomainGroup = domain("www.example.com", "news.example.com", "ad.example.com")
            OR domain_suffix("cdn.example.com")
DestPortGroup = port(443)
```

整体语义：

```text
AND(
  OR(DomainGroup),
  OR(DestPortGroup)
)
```

## 约束

- rule_set 不嵌套展开另一个 rule_set。
- 多个 rule_set 分支之间保持 OR。
- 合并后的规则仍必须保留 first-match 顺序。

## 测试要点

- 合并不产生笛卡尔积。
- 同 group 条件能正确 union。
- 不同 group 条件保持 AND。
- 多分支 rule_set 保持 OR。
- first-match 顺序不变。
