# Feature Spec: Review Session

## Goal

用户可以开始一次今日复习，逐个查看到期单词，并根据记忆结果更新调度。

## Non-goals

- 不做复杂记忆算法。
- 不做语音评测。
- 不做 AI 出题。

## User Flows

| Flow | Steps | Expected Result |
|---|---|---|
| 开始复习 | 点击今日复习 | 显示第一个 due word |
| 标记 remembered | 查看释义后点击 remembered | 更新 nextReviewAt |
| 标记 forgotten | 点击 forgotten | 单词进入 mistake list |
| 完成 session | 所有 due words 完成 | 显示 session summary |

## State Transitions

```text
due -> reviewing -> remembered -> scheduled
due -> reviewing -> forgotten -> mistake-list + scheduled
```

## Data Model Additions

```text
ReviewState
- wordId
- lastReviewedAt
- nextReviewAt
- intervalDays
- mistakeCount
- rememberedCount
```

## Acceptance Criteria

- [ ] 今日复习只显示 due words。
- [ ] remembered 会增加 interval。
- [ ] forgotten 会重置 interval 为 1 day。
- [ ] forgotten 会增加 mistakeCount。
- [ ] session 结束后显示 summary。

