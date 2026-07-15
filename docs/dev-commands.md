# Dev Commands

Run with `mise run <task>` or `mise <task>`.

## Root

- `setup` - full project setup (tools, hooks, deps, build parser, Playwright)
- `config_fmt` - format config and Markdown with `dprint`

## Parser (`parser/`)

- `parser:fmt` - format code
- `parser:lint` - lint code
- `parser:test` - run tests
- `parser:build` - build parser

## Backend (`backend/`)

- `backend:sync` - `uv sync`
- `backend:fmt` - format code
- `backend:lint` - lint code
- `backend:test` - run tests
- `backend:dev` - run FastAPI dev server

## Frontend (`frontend/`)

- `frontend:sync` - `pnpm install`
- `frontend:lint` - format and lint code
- `frontend:test` - run tests
- `frontend:e2e` - run E2E tests (Playwright)
- `frontend:build` - build frontend
- `frontend:dev` - run Next.js dev server

You can also run `cargo` / `uv` / `pnpm` directly from each subproject directory.
