use serde::Serialize;
use std::fmt;

/// Source of game data: either the core game or a specific mod.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub enum Source {
    Core,
    #[allow(unfulfilled_lint_expectations)]
    #[expect(dead_code)]
    Mod(String),
}

impl Source {
    #[must_use]
    pub const fn core() -> Self {
        Self::Core
    }

    #[must_use]
    #[allow(unfulfilled_lint_expectations)]
    #[expect(dead_code)]
    pub fn mod_named(name: impl Into<String>) -> Self {
        Self::Mod(name.into())
    }

    #[must_use]
    pub const fn name(&self) -> &str {
        match self {
            Self::Core => "core",
            Self::Mod(name) => name.as_str(),
        }
    }
}

impl fmt::Display for Source {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Core => write!(f, "core"),
            Self::Mod(name) => write!(f, "mod({name})"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod source_tests {
        use super::*;

        #[test]
        fn test_source_core() {
            let source = Source::core();
            assert_eq!(source.name(), "core");
            assert_eq!(source.to_string(), "core");
        }

        #[test]
        fn test_source_mod() {
            let source = Source::mod_named("test_mod");
            assert_eq!(source.name(), "test_mod");
            assert_eq!(source.to_string(), "mod(test_mod)");
        }

        #[test]
        fn test_source_equality() {
            assert_eq!(Source::core(), Source::core());
            assert_eq!(Source::mod_named("a"), Source::mod_named("a"));
            assert_ne!(Source::core(), Source::mod_named("a"));
        }
    }
}
