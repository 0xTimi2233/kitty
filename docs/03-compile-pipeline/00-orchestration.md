# Compile Pipeline 编排

推荐顺序：

1. log bootstrap。
2. config decode。
3. schema normalize。
4. reload diff 预判。
5. basic validate。
6. semantic validate。
7. loop check。
8. rule_set load/cache。
9. rule_set decode/verify。
10. rule_set expand/merge。
11. priority flattening。
12. string interning。
13. match index compile。
14. runtime publish plan。
15. listener apply plan。
16. atomic publish。

外部 reload 必须先做轻量 diff。仅日志变化时不触发全量编译。
