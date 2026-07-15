# Testing

## Organization

- Tests for a function/class/struct must be separated from other tests:
  - **Rust**: each in its own `mod`.
  - **Python**: each in its own class (`Test<CamelCaseName>`).
  - **TypeScript**: each in its own `describe` block.

## Patterns

- Follow Arrange-Act-Assert. Separate each block with an empty line — no explicit comments.
- The optional fourth teardown block should be separated with an empty line too, if present.
- Don't test what the language/library provides (`derive` in Rust, parent classes in Python).

## Naming, file structure and language-specific patterns

### Python

- Tests for `app/utils.py` -> `tests/test_utils.py`.
- Avoid autouse fixtures.
- No return type annotations needed on tests.

### TypeScript

- Tests for `app/utils.ts(x)` -> `__tests__/utils.test.ts(x)`.
- Component tests: include snapshot tests. Page tests: no snapshots.
- Use `test`, not `it`.
- Reset mocks after each test, not before.
