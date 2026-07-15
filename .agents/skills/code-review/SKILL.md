---
name: code-review
description: Review a code diff for issues and produce a PR description. Use when asked to review a PR, review code, review a diff, or generate a PR description/commit message.
---

## 1. Get the Diff

Acquire the diff to review. Prefer `git diff main...HEAD` for PR-scoped diffs. If the user specifies
a branch or commit range, use that instead. Read changed files in full when context is needed to
evaluate the diff.

If the user described the purpose of the changes, use that context. Otherwise, deduce the purpose
from the diff itself.

## 2. Review

Analyze the diff for:

- **Correctness** — bugs, logic errors, off-by-one, unhandled edge cases
- **Security** — injection, secrets, unsafe operations
- **Consistency** — naming, patterns (including documentation hierarchy), and conventions matching
  the surrounding codebase
- **Scope** — unrelated changes mixed in; keep PRs focused
- **Missing updates** — do README, docs, AGENTS.md, or tests need updating?
- **Readability** — unnecessary complexity, unclear names, overly dense logic

Do not run linters, type checkers, or tests. CI handles that. Never make code changes. Describe what
should change.

Ensure changes adhere to conventions in `docs/code-review.md`.

## 3. Output

### Review Report

List issues and suggestions grouped by severity:

- **Must fix** — bugs, security issues, broken logic
- **Should fix** — readability, missing edge cases, inconsistent patterns
- **Consider** — minor style, optional improvements

Reference specific file paths and line numbers. Be concrete: say _what_ is wrong and _why_, not just
"could be better".

### PR Description / Squashed Commit Message

Write a single cohesive description covering:

- **What** changed (summary of the diff purpose)
- **Why** it changed (the motivation or problem being solved)
- **How** it works (brief technical approach, only if non-obvious)

Keep it to one paragraph plus bullet points for secondary changes. The description should be useful
as both a PR body and a squashed merge commit message.
