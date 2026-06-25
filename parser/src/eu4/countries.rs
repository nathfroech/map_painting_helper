use serde::{Deserialize, Serialize};

/// A parsed country tag entry.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct CountryTag {
    pub tag: String,
    pub path: String,
}
