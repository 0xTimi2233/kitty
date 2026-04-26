# Compile Pipeline 编排

Compile Pipeline 是控制面 full compile 的标准路径，从 `config-decode` 开始，不包含进程启动日志 bootstrap。

顺序：

```text
01 config-decode
02 basic-validate
03 structural-normalize
04 semantic-collect
05 semantic-validate
06 loop-check
07 rule-set-reference-prune
08 rule-set-load-cache
09 rule-set-decode-verify
10 rule-set-expand-merge
11 priority-flattening
12 string-interning
13 match-index-compile
14 runtime-plan
15 listener-apply-plan
16 atomic-publish
17 post-publish-maintenance
```

reload diff precheck 属于 operations/reload controller，不属于 full compile 节点。
