# Compile Pipeline 编排

Compile Pipeline 的目标是把用户配置和规则集编译成只读、可原子发布、可被 Match Pipeline 高速访问的 Runtime Model。

## 触发来源

- `start`：首次启动，必须完成完整编译和发布。
- `external reload`：用户显式触发 reload，先做轻量差异判断，再决定是否全量编译。
- `internal rule_set refresh`：规则集自动刷新任务触发。刷新失败时保留旧 Runtime Model。

## 阶段顺序

1. `01-log-bootstrap.md`：初始化最小日志能力。
2. `02-config-decode.md`：反序列化用户配置。
3. `03-schema-normalize.md`：执行 schema 层 normalize。
4. `04-reload-diff-precheck.md`：在重编译前做轻量差异预判。
5. `05-basic-validate.md`：执行基础校验。
6. `06-semantic-validate.md`：执行引用解析和语义校验。
7. `07-loop-check.md`：检查 resolver、detour、outbound、rule_set 相关回环。
8. `08-rule-set-reference-prune.md`：计算实际被引用的 rule_set。
9. `09-rule-set-load-cache.md`：读取、下载、刷新或复用 rule_set 缓存。
10. `10-rule-set-decode-verify.md`：解码 source rule_set 或验证 KSR 文件头。
11. `11-rule-set-expand-merge.md`：展开 rule_set 并合并到规则 IR。
12. `12-priority-flattening.md`：固化优先级覆盖与 action target。
13. `13-string-interning.md`：驻留重复字符串和 tag。
14. `14-match-index-compile.md`：编译倒排索引与 bitmap 数据。
15. `15-runtime-plan.md`：生成 Runtime Model 与发布计划。
16. `16-listener-apply-plan.md`：生成 listener apply plan。
17. `17-atomic-publish.md`：原子发布 Runtime Model。

## 编排原则

- `external reload` 必须在重编译前完成轻量差异判断。
- `no-op reload` 不发布新 Runtime Model。
- `log-only reload` 只替换日志配置，不触发全量编译。
- 规则集、规则、索引、运行态模型的结构性变化必须全量编译。
- 编译阶段可以使用复杂结构，运行态必须是只读、低分支、可快速访问的结构。
- 所有 action target、server reference、outbound reference 都必须在编译期解析为稳定 ID。
