# Rule IR Model

规则先从 schema 转换为 IR，再进入展开、合并和索引编译。

IR 只保留：

- `and`
- `or`
- `not`
- `leaf`

```text
RuleExpr = And(Vec<RuleExpr>)
         | Or(Vec<RuleExpr>)
         | Not(Box<RuleExpr>)
         | Leaf(ConditionSet)
```

## condition group 语义

同一组内 OR，不同组之间 AND：

| 组               | 字段示例                                                   | 语义            |
|-----------------|--------------------------------------------------------|---------------|
| DomainGroup     | domain / domain_suffix / domain_keyword / domain_regex | 组内 OR；空表示 ANY |
| DestIpGroup     | ip_cidr / ip_is_private                                | 组内 OR         |
| DestPortGroup   | port / port_range                                      | 组内 OR         |
| SourceIpGroup   | source_ip_cidr / source_ip_is_private                  | 组内 OR         |
| SourcePortGroup | source_port / source_port_range                        | 组内 OR         |
| ProcessGroup    | process_name / process_path / process_path_regex       | 组内 OR         |
| ...             | ...                                                    | 组内 OR         |

默认规则和 logical 规则在 IR 中统一表达。rule_set 引用在展开阶段替换为可合并的分支。

- 组内 OR。
- 组间 AND。
- 无条件组表示 ANY。
- 多个 rule_set 分支保持 OR。
- 分支与外层规则做 AND 合并。