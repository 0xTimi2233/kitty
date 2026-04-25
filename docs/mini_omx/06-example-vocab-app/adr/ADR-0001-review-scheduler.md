# ADR-0001: MVP Review Scheduler

## Status

Accepted

## Context

MVP 需要一个可解释、可测试的复习调度策略。完整 SM-2 或 FSRS 会带来更多参数、迁移和测试成本。

## Decision

MVP 使用 simple spaced repetition：

| Result | Next Interval |
|---|---|
| forgotten | 1 day |
| remembered first time | 2 days |
| remembered repeatedly | previous interval * 2, capped at 30 days |

## Alternatives

| Option | Pros | Cons | Decision |
|---|---|---|---|
| SM-2 | 成熟 | 参数更多，MVP 验证成本高 | rejected |
| FSRS | 效果更好 | 实现复杂，需要更多数据 | rejected |
| Simple rule | 可解释，可测试 | 个性化弱 | accepted |

## Consequences

- 数据模型需要保存 `lastReviewedAt`、`nextReviewAt`、`intervalDays`、`mistakeCount`。
- 后续可以通过 migration 升级为 SM-2 或 FSRS。

## Validation

- 用 unit tests 覆盖 remembered / forgotten 的状态转移。
- 用 integration tests 覆盖 review session 完成后队列变化。

