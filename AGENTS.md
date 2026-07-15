# Project

Three-package monorepo for Paradox Interactive game data tooling.

| Package     | Language   | Entrypoint                          |
| ----------- | ---------- | ----------------------------------- |
| `parser/`   | Rust       | `src/main.rs` (bin)                 |
| `backend/`  | Python     | `app/main.py` (FastAPI)             |
| `frontend/` | TypeScript | `src/app/` (Next.js, static export) |

## Tools

- **mise** — task runner and tool version manager. Run from project root: `mise run <task>`.
- **prek** — git hook manager. Run `prek run --all-files` to check all hooks.

## Ground Rules

- **Ask, don't assume** — when uncertain about implementation, ask the developer. During code review
  you may suggest additional changes (but never make them yourself).
- **Documentation hierarchy**: 1) No inline comments unless explaining _why_. 2) Docstrings for
  important public APIs (not required in general). 3) Separate docs as a last resort, describing
  global concepts.

## Branching

- Prefixes: `feature/`, `bugfix/`, or `chore/`.
- Direct commits to `main` are prohibited.

## Detailed guides

- [Dev Commands](docs/dev-commands.md) — all mise tasks
- [Architecture](docs/architecture.md) — parser design and parsing logic
- [Testing](docs/testing.md) — conventions for tests in Rust, Python, TypeScript
- [Code Review](docs/code-review.md) — review guidelines
