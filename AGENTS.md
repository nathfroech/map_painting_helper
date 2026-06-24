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

## Architecture

### Parser

The parser is designed to handle multiple Paradox Interactive games and their mods.

- `common/`: Contains base types and traits.
  - `Source`: Enum representing the data source (Core game or a Mod).
  - `Parser`: Trait providing common logic for file and directory parsing using the `jomini` crate.
- `eu4/`: EU4-specific parsing logic.
  - `EU4Parser`: Implements the `Parser` trait and stores data in a structured `store`.
- `store`: A nested map (`Source` -> `DataType` -> `FileName` -> `Entries`) that preserves all data
  from all sources, allowing for merging logic to be implemented in the UI.

## Rules

### Implementation

- Never assume what is needed or what is better. If there are multiple ways to implement something,
  and it is not clear from code, comments or documentation, which option to choose - ask the
  developer to make a choice.
- The code should be self-explanatory and ideally have little to no comments. If there are inline
  comments, they must answer the question "why", not "what" or "how". (Technical comments like
  inline ignores are, of course, acceptable too.) Docstrings for classes or functions are not
  required in general but are fine for some important parts of code. Separate documentation files
  should be a last resort and should mostly describe global concepts.

#### Parsing Logic

1. Each game directory (for example, `country_tags`) may contain multiple files of the same
   structure/data type.
2. There will be mods, not only the base game. If a mod has a file with the same name and path as
   the base game, it overrides the base game's file.
3. If multiple mods override the same file, the latest mod wins.
4. However, parser should still keep all the data - the merging should be done via UI by selecting
   certain mods.
5. Parse result is a JSON file with structured data.

### Testing

- A general rule of thumb: if there is a function or method, there should be at least one test for
  it. Enums or classes/structs without methods usually do not require tests.
- Tests for a certain function/class/struct should be separated from other tests. In case of Rust
  this means being withing their own `mod`; in case of Python - test class.
- Do not test features that are implemented within a language or used library (`derive` in Rust,
  parent classes in Python, etc.)
- Tests should follow the Arrange-Act-Assert pattern. Each block should be separated by an empty
  line - no explicit comments!

### Code Review

- No matter if the main theme of the Pull Request is given or was figured out - pay attention to the
  balance of changes. It is fine to have a small set of unrelated changes (because they were
  forgotten in previous PRs or indirectly make the work on the current one easier). It may be fine
  to have a big refactoring, followed by the changes in logic, if they touch the same places. But
  situations when there is a big piece of completely unrelated changes are undesirable.
- Try to imagine if there are any additional changes that might fit the main theme of the PR and
  could be useful in the future.
- If there were any code changes, check if they require to be reflected in `README.md`,
  documentation or `AGENTS.md`.
- Linters, type checking and tests should not be executed during the review - there is a CI job for
  that.
- No code changes should be made during the review. If they are necessary, they should only be
  described in the output.
