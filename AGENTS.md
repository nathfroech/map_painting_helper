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

## Branching Policy

- Use `feature/`, `bugfix/`, or `chore/` prefixes for branch names.
- Direct commits to `main` are prohibited.

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
  - `frontend:lint` - format and lint code
  - `frontend:test` - run tests
  - `frontend:e2e` - run E2E tests (Playwright)
  - `frontend:build` - build frontend
  - `frontend:dev` - run Next.js dev server

It is also possible to run corresponding commands (`cargo` / `uv` / `pnpm`) directly from the
subproject dirs.

## Implementing new features

- Never assume what is needed or what is better. If there are multiple ways to implement something,
  and it is not clear from code, comments or documentation, which option to choose - ask the
  developer to make a choice.

## Testing rules

- A general rule of thumb: if there is a function or method, there should be at least one test for
  it. Enums or classes/structs without methods usually do not require tests.
- Tests for a certain function/class/struct should be separated from other tests. In case of Rust
  this means being withing their own `mod`; in case of Python - test class.
- Do not test features that are implemented within a language or used library (`derive` in Rust,
  parent classes in Python, etc.)

## Parsing Logic

1. Each game directory (for example, `country_tags`) may contain multiple files of the same
   structure/data type.
2. There will be mods, not only the base game. If a mod has a file with the same name and path as
   the base game, it overrides the base game's file.
3. If multiple mods override the same file, the latest mod wins.
4. However, parser should still keep all the data - the merging should be done via UI by selecting
   certain mods.
5. Parse result is a JSON file with structured data.
