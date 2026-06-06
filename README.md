<div align="center">

# Map Painting Helper

**Toolchain for parsing, serving, and visualizing Paradox Interactive game data.**

</div>

## Tech Stack

- **Parser** — Rust
- **Backend** — FastAPI
- **Frontend** — TypeScript / Next.js

## Setup

You need [mise](https://mise.jdx.dev/) for installing and managing all tools (Rust, Python, Node,
etc.)

```sh
git clone <repo-url> && cd map_painting_helper
mise run setup
```

This will install all tools, set up git hooks, and sync every subproject's dependencies.

### Windows — MinGW Linker

Rust on Windows needs a GNU linker. Install [MinGW64](https://www.mingw-w64.org/) first. Then create
`mise.local.toml` in the project root with the following content (change paths to your MinGW
installation):

```toml
[env]
CARGO_BUILD_TARGET = "x86_64-pc-windows-gnu"
CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER = "C:\\msys64\\mingw64\\bin\\gcc.exe"
_.path = ["C:\\msys64\\mingw64\\bin"]
```

## Common Commands

All commands run from the project root. Tasks are namespaced by subproject; you can also `cd` into a
subdirectory and run short names (e.g. `mise run test`). You can also run subproject-specific
commands (`cargo`/`uv`/`pnpm`) directly in subdirectories.

### Parser (`parser/`)

| Command                 | Action           |
| ----------------------- | ---------------- |
| `mise run parser:build` | Build the parser |
| `mise run parser:test`  | Run all tests    |
| `mise run parser:lint`  | Lint with Clippy |
| `mise run parser:fmt`   | Format code      |

### Backend (`backend/`)

| Command                      | Action               |
| ---------------------------- | -------------------- |
| `mise run backend:sync`      | Sync dependencies    |
| `mise run backend:dev`       | Start dev server     |
| `mise run backend:test`      | Run all tests        |
| `mise run backend:lint`      | Lint & fix with Ruff |
| `mise run backend:typecheck` | Type-check with Ty   |

### Frontend (`frontend/`)

| Command                       | Action                 |
| ----------------------------- | ---------------------- |
| `mise run frontend:sync`      | Sync dependencies      |
| `mise run frontend:dev`       | Start dev server       |
| `mise run frontend:build`     | Production build       |
| `mise run frontend:test`      | Unit tests (Vitest)    |
| `mise run frontend:e2e`       | E2E tests (Playwright) |
| `mise run frontend:lint`      | Lint & fix with Biome  |
| `mise run frontend:typecheck` | Type-check with tsc    |

### Config & Hooks

| Command                | Action                          |
| ---------------------- | ------------------------------- |
| `mise run config_fmt`  | Format config files with dprint |
| `prek run --all-files` | Run git hooks on all files      |

## Branching Policy

This project enforces a branch naming convention via git hooks:

- Use `feature/`, `bugfix/`, or `chore/` prefixes for branch names (e.g., `feature/add-login`).
- Direct commits to `main` are prohibited.

## License

MIT — see [LICENSE](./LICENSE)
