# Test Plan: Review Session

## Unit Tests

| Case | Input | Expected |
|---|---|---|
| forgotten | interval 8 | next interval 1 |
| remembered first time | interval empty | next interval 2 |
| remembered repeatedly | interval 4 | next interval 8 |
| interval cap | interval 30 | remains 30 |

## Integration Tests

| Case | Setup | Expected |
|---|---|---|
| due queue | words with mixed nextReviewAt | only due words shown |
| session summary | complete 3 words | summary counts remembered/forgotten |

## E2E Tests

| Case | Steps | Expected |
|---|---|---|
| complete review | add words -> start review -> mark results -> finish | schedule updated and summary shown |

## Commands

```bash
npm test
npm run build
```

## Pass / Fail Criteria

Review scheduling, queue filtering, and session summary behavior all pass.

