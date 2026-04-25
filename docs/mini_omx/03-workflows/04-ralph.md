# Workflow: Ralph

用途：一个 owner 持续执行一个 approved task，直到完成或明确 blocked。借鉴 OMX `$ralph`，但通过 prompt 和 status 文件实现。

## 前置条件

- task 已经在 `status.md` 中标记为 `approved`。
- task 有 spec 和 test-plan。
- task 有明确 write scope。

## 步骤

1. 主线程选择一个 approved task。
2. Engineer 读取：
   - project AGENTS.md
   - workflow
   - task spec
   - task test-plan
   - status
3. Engineer 先补测试或验证基线。
4. Engineer 实现最小可审查 diff。
5. Engineer 运行 test-plan 中的相关命令。
6. 如果失败：
   - 记录 failure
   - 做 root cause
   - 在 scope 内修复
   - 重跑验证
7. 如果同一失败重复 3 次，停止并标记 blocked。
8. Verifier 只读验证。
9. 主线程更新 `status.md`。

## 完成标准

必须同时满足：

- spec 中的 acceptance criteria 全部满足。
- test-plan 中的 required commands 通过，或写明无法运行原因。
- 没有未记录的 scope change。
- status 已更新。

## 输出

- changed files
- commands run
- pass/fail result
- unresolved risks
- next recommended task

