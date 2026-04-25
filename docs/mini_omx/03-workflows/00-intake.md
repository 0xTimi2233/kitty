# Workflow: Intake

用途：在写代码前澄清需求。适合新项目、新 feature、需求模糊或边界不清的任务。

## 输入

- 用户原始想法
- 现有产品文档
- 现有约束和非目标

## 角色

- 主线程：orchestrator
- PM：主责
- Architect：只回答架构边界问题
- QA：只补验收标准和测试风险

## 步骤

1. 主线程读取项目 `AGENTS.md` 和本 workflow。
2. 主线程创建或选择目标文档目录，例如 `docs/product/`。
3. PM 澄清：
   - 用户是谁
   - 核心 job-to-be-done 是什么
   - MVP 包含什么
   - 明确不做什么
   - 成功标准是什么
4. Architect 检查需求是否隐含架构决策。
5. QA 补充可测试验收标准。
6. 主线程整合结果，写入：
   - `vision.md`
   - `requirements.md`
   - `non-goals.md`
   - `acceptance.md`

## 输出格式

必须包含：

- Product goal
- Users
- MVP scope
- Non-goals
- Acceptance criteria
- Open questions
- Planning blockers

## 停止条件

出现以下情况时不要进入 planning：

- 用户目标仍不明确。
- MVP 和 non-goals 冲突。
- 验收标准无法测试。
- 关键架构决策无 owner。

