# Test Plan: Wordbook CRUD

## Unit Tests

| Case | Input | Expected |
|---|---|---|
| create wordbook | valid name | success |
| create wordbook | empty name | validation error |
| add word | valid word | success |
| add duplicate word | same wordbook | duplicate handling |

## Integration Tests

| Case | Setup | Expected |
|---|---|---|
| wordbook list refresh | create then list | new wordbook visible |
| delete wordbook | existing wordbook | removed from list |

## E2E Tests

| Case | Steps | Expected |
|---|---|---|
| manage words | create wordbook -> add word -> edit word -> delete word | all UI states update |

## Commands

```bash
npm test
npm run build
```

## Pass / Fail Criteria

All validation, CRUD, and UI flow tests pass.

