# Architecture

## Parser

The parser handles multiple Paradox Interactive games and their mods.

- `common/`: Base types and traits.
  - `Source`: Enum for data source (Core game or Mod).
  - `Parser`: Trait for file/directory parsing via the `jomini` crate.
- `eu4/`: EU4-specific parsing.
  - `EU4Parser`: Implements `Parser`, stores data in a structured `store`.
- `store`: Nested map (`Source` → `DataType` → `FileName` → `Entries`) preserving all data from all
  sources for UI-driven merging.

## Parsing Logic

1. Each game directory (e.g. `country_tags`) may contain multiple files of the same structure.
2. Mods override base game files when paths match.
3. When multiple mods override the same file, the latest mod wins.
4. The parser keeps all data — merging is done in the UI by selecting mods.
5. Parse output is a JSON file with structured data.
