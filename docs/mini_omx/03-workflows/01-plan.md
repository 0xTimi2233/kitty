# Workflow: Plan

用途：把澄清后的需求转成可执行 roadmap、ADR、spec 和 test-plan 列表。只规划，不实现。

## 输入

- `vision.md`
- `requirements.md`
- `non-goals.md`
- `acceptance.md`
- 现有 ADR / specs / status

## 角色

- PM：milestone、priority、status
- Architect：ADR、架构边界、dependency policy
- QA：test-plan 要求和 verification commands
- 主线程：整合和冲突裁决

## 步骤

1. PM 把需求拆成 milestone。
2. Architect 标记需要 ADR 的决策。
3. Architect 给每个 feature 定义技术边界：
   - data model
   - module boundary
   - API contract
   - persistence / external dependency
4. QA 给每个 feature 定义测试层级：
   - unit
   - integration
   - e2e
   - manual
5. PM 生成 task table：
   - id
   - milestone
   - dependency
   - owner role
   - spec path
   - test-plan path
   - status
6. 主线程检查：
   - 是否有循环依赖
   - 是否有未分配 owner
   - 是否有 task 没有 spec/test-plan
   - 是否有 task scope 太大

## 输出

- `roadmap.md`
- `adr/ADR-000N-*.md`
- `specs/<feature>.md`
- `specs/<feature>-test-plan.md`
- `status.md`

## Approved Task Definition

一个 task 只有满足以下条件才能标记为 `approved`：

- 有明确 spec。
- 有 test-plan。
- 有 write scope。
- 有 acceptance criteria。
- 有 verification commands。
- 没有 unresolved architecture blocker。

