# Workflow: Verify

用途：只读验证实现是否满足文档契约。

## 角色

- Verifier：证据化验证
- Reviewer：风险审查
- 主线程：汇总结论，不擅自扩大 scope

## 步骤

1. 读取 task spec。
2. 读取 test-plan。
3. 读取 status 和 worker report。
4. 检查 changed files 是否在 write scope 内。
5. 对照 acceptance criteria。
6. 检查测试是否覆盖 test-plan 的 required cases。
7. 运行或复核 verification commands。
8. Reviewer 做 correctness / security / maintainability / missing tests 审查。
9. 输出 verdict。

## Verdict

- `PASS`：可合并或可进入下一个 task。
- `PARTIAL`：核心行为完成，但有明确缺口。
- `FAIL`：不满足 spec/test-plan，必须返工。

## 输出格式

```text
Verdict: PASS | PARTIAL | FAIL

Evidence:
- ...

Commands:
- ...

Findings:
- ...

Remaining risks:
- ...
```

