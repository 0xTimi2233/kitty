# 背单词 Web App Vision

## Product Goal

构建一个个人使用的背单词 Web App，帮助用户创建单词本、安排复习、记录掌握情况，并通过简单的 spaced repetition 提高记忆效率。

## Users

| User | Need | Success Signal |
|---|---|---|
| 自学英语用户 | 管理自己的单词和复习进度 | 每天能看到今日复习队列 |
| 考试备考用户 | 快速添加考试词汇并跟踪错词 | 错词能自动进入重点复习 |

## MVP Scope

| Feature | Included | Reason |
|---|---|---|
| Wordbook CRUD | yes | 核心数据入口 |
| Word CRUD | yes | 用户必须能维护单词 |
| Review session | yes | 核心学习流程 |
| Simple spaced repetition | yes | MVP 记忆调度 |
| Mistake list | yes | 提升复习效率 |
| User account | no | MVP 先做 local-first |
| Cloud sync | no | 复杂度高，MVP 暂不需要 |
| AI example generation | no | 后续增强功能 |

## Acceptance Criteria

- [ ] 用户可以创建、编辑、删除单词本。
- [ ] 用户可以添加单词、释义、例句和标签。
- [ ] 用户可以开始今日复习。
- [ ] 用户可以把单词标记为 remembered 或 forgotten。
- [ ] forgotten 单词进入错词列表。
- [ ] 系统根据复习结果更新下次复习日期。

