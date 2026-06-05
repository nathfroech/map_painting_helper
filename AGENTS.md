## Repository

Three-package monorepo for Paradox Interactive game data tooling.

| Package     | Language   | Entrypoint                          |
| ----------- | ---------- | ----------------------------------- |
| `parser/`   | Rust       | `src/main.rs` (bin)                 |
| `backend/`  | Python     | `app/main.py` (FastAPI)             |
| `frontend/` | TypeScript | `src/app/` (Next.js, static export) |

## Setup & Tools

- **mise** — task runner and tool version manager. Run everything from project root:
  `mise run <task>`.
- **prek** — git hook manager. Run `prek run --all-files` to check all hooks on all files.

## Dev Commands (usable from root or subproject dirs)

`mise` tasks can be executed with `mise run <task>` or `mise <task>`.

- root tasks:
  - `setup` — full project setup (install tools, hooks, deps, build parser, install Playwright).
  - `config_fmt` — format config files and Markdown with `dprint`.
- parser tasks (executed in `parser/` dir):
  - `parser:fmt` - format code
  - `parser:lint` - lint code
  - `parser:test` - run tests
  - `parser:build` - build parser
- backend tasks (executed in `backend/` dir):
  - `backend:sync` - `uv sync`
  - `backend:fmt` - format code
  - `backend:lint` - lint code
  - `backend:test` - run tests
  - `backend:dev` - run FastAPI dev server
- frontend tasks (executed in `frontend/` dir):
  - `frontend:sync` - `pnpm install`
  - `frontend:fmt` - format code
  - `frontend:lint` - lint code
  - `frontend:test` - run tests
  - `frontend:e2e` - run E2E tests (Playwright)
  - `frontend:build` - build frontend
  - `frontend:dev` - run Next.js dev server

It is also possible to run corresponding commands (`cargo` / `uv` / `pnpm`) directly from the
subproject dirs.
