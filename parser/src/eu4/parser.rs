use crate::common::parser::Parser;
use crate::common::types::Source;
use crate::eu4::countries::CountryTag;
use serde_json;
use std::collections::HashMap;
use std::path::PathBuf;

pub struct EU4Parser {
    content_dir: PathBuf,
    pub(crate) store: HashMap<String, HashMap<String, HashMap<String, Vec<serde_json::Value>>>>,
}

impl Parser for EU4Parser {}

impl EU4Parser {
    #[must_use]
    pub fn new(content_dir: PathBuf) -> Self {
        Self { content_dir, store: HashMap::new() }
    }

    /// Insert a data entry, keyed by its source and originating filename.
    fn insert_data(
        &mut self,
        source: &Source,
        data_type: impl Into<String>,
        file_name: impl Into<String>,
        data: serde_json::Value,
    ) {
        self.store
            .entry(source.name().to_string())
            .or_default()
            .entry(data_type.into())
            .or_default()
            .entry(file_name.into())
            .or_default()
            .push(data);
    }

    /// Parse `common/country_tags/*.txt` files.
    ///
    /// Each file contains entries of the form `TAG = "path/to/file.txt"`.
    /// Files are parsed as tag → path maps and stored keyed by their stem.
    ///
    /// # Errors
    ///
    /// Returns an IO error if files cannot be read, or a jomini error if
    /// parsing fails.
    pub fn parse_country_tags(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Parsing country tags");
        let dir_path = self.content_dir.join("common").join("country_tags");
        if !dir_path.exists() {
            return Ok(());
        }
        let files = self.parse_directory::<HashMap<String, String>>(&dir_path, "txt")?;
        for (stem, tag_maps) in files {
            for map in tag_maps {
                let mut countries: Vec<CountryTag> =
                    map.into_iter().map(|(tag, path)| CountryTag { tag, path }).collect();
                countries.sort_by(|a, b| a.tag.cmp(&b.tag));
                self.insert_data(
                    &Source::core(),
                    "country_tags",
                    stem.clone(),
                    serde_json::to_value(countries)?,
                );
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn create_eu4_parser_with_country_tags(content: &str) -> (tempfile::TempDir, EU4Parser) {
        let temp_dir = tempfile::tempdir().expect("failed to create temp dir");
        let country_tags_dir = temp_dir.path().join("common").join("country_tags");
        std::fs::create_dir_all(&country_tags_dir).expect("failed to create country_tags dir");
        let file_path = country_tags_dir.join("00_countries.txt");
        let mut file = std::fs::File::create(&file_path).expect("failed to create file");
        file.write_all(content.as_bytes()).expect("failed to write content");
        let parser = EU4Parser::new(temp_dir.path().to_path_buf());
        (temp_dir, parser)
    }

    #[test]
    fn test_parse_country_tags_non_existent() {
        let temp_dir = tempfile::tempdir().unwrap();
        let mut parser = EU4Parser::new(temp_dir.path().to_path_buf());

        parser.parse_country_tags().unwrap();

        assert!(parser.store.is_empty());
    }

    #[test]
    fn test_parse_country_tags_with_comments() {
        let content = concat!(
            "AAA = \"countries/Aachen.txt\"\n",
            "# This is a comment\n",
            "ABB = \"countries/Air.txt\"\n",
            "\n",
            "TUR = \"countries/Turkey.txt\" # Another comment\n",
        );
        let (_keep, mut parser) = create_eu4_parser_with_country_tags(content);

        parser.parse_country_tags().unwrap();

        let tags = &parser.store["core"]["country_tags"]["00_countries"];
        assert_eq!(tags.len(), 1);
        let entries = tags[0].as_array().unwrap();
        assert_eq!(entries.len(), 3);
        assert_eq!(entries[0]["tag"], "AAA");
        assert_eq!(entries[0]["path"], "countries/Aachen.txt");
        assert_eq!(entries[1]["tag"], "ABB");
        assert_eq!(entries[1]["path"], "countries/Air.txt");
        assert_eq!(entries[2]["tag"], "TUR");
        assert_eq!(entries[2]["path"], "countries/Turkey.txt");
    }

    #[test]
    fn test_parse_country_tags_empty_file() {
        let (_keep, mut parser) = create_eu4_parser_with_country_tags("");

        parser.parse_country_tags().unwrap();

        let data = &parser.store;
        // There is a single empty file, so we didn't really write anything
        assert_eq!(data.len(), 0);
    }

    #[test]
    fn test_store_serializes_with_core_and_mod_sources() {
        let temp_dir = tempfile::tempdir().unwrap();
        let mut parser = EU4Parser::new(temp_dir.path().to_path_buf());

        parser.insert_data(&Source::core(), "country_tags", "core_file", serde_json::json!([]));
        parser.insert_data(
            &Source::mod_named("mod_x"),
            "country_tags",
            "mod_file",
            serde_json::json!([]),
        );

        let serialized = serde_json::to_string_pretty(&parser.store);

        assert!(serialized.is_ok());
        let json = serialized.unwrap();
        assert!(json.contains("\"core\""));
        assert!(json.contains("\"mod_x\""));
    }
}
