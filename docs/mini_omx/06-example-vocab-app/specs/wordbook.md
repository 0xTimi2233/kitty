# Feature Spec: Wordbook CRUD

## Goal

用户可以创建、查看、编辑、删除单词本，并在单词本中维护单词。

## Non-goals

- 不做账号系统。
- 不做云同步。
- 不做多人共享单词本。

## User Flows

| Flow | Steps | Expected Result |
|---|---|---|
| 创建单词本 | 输入名称 -> 保存 | 列表出现新单词本 |
| 添加单词 | 进入单词本 -> 新增 word/definition/example | 单词出现在单词列表 |
| 删除单词 | 点击删除 -> 确认 | 单词从列表移除 |

## Data Model

```text
Wordbook
- id
- name
- description
- createdAt
- updatedAt

Word
- id
- wordbookId
- text
- definition
- example
- tags
- createdAt
- updatedAt
```

## Error Cases

| Case | Expected Behavior | User-facing Message |
|---|---|---|
| wordbook name empty | reject save | 请输入单词本名称 |
| duplicated word in same wordbook | reject or merge by product decision | 单词已存在 |
| deleting non-empty wordbook | require confirm | 删除后无法恢复 |

## Acceptance Criteria

- [ ] 可以创建 wordbook。
- [ ] 可以编辑 wordbook name。
- [ ] 可以删除 wordbook。
- [ ] 可以添加 word。
- [ ] 可以编辑 word。
- [ ] 可以删除 word。

