# Workflow: Spec And Test Plan

用途：为一个 feature 生成 agent 可执行的 spec 和 test-plan。

## 角色

- Architect：写 technical contract
- QA：写 test matrix
- PM：检查 scope 是否仍属于 roadmap

## Spec 必须包含

- Goal
- Non-goals
- User flows
- Data model
- API contract
- State transitions
- Error cases
- Security and privacy considerations
- Performance considerations
- Dependencies
- Open questions

## Test Plan 必须包含

- Unit tests
- Integration tests
- E2E tests
- Manual verification
- Regression cases
- Edge cases
- Commands
- Pass/fail criteria

## 规则

- spec 是 Engineer 的输入 contract，不是 brainstorm。
- test-plan 是 Verifier 的验收依据，不是建议列表。
- 未确定内容必须进入 `Open questions`，不能隐式交给 Engineer 猜。

