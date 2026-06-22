use jomini;
use serde;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::{fs, io};

/// Text encoding used by a game's plaintext data files.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Encoding {
    #[allow(dead_code)]
    Windows1252,
    #[allow(dead_code)]
    Utf8,
}

/// A parser for Paradox game text files.
///
/// Knows which encoding to use and provides all the logic for walking
/// directories, reading files, and deserializing content.
pub trait Parser {
    /// Return encoding based on the current OS.
    #[must_use]
    fn get_encoding(&self) -> Encoding {
        #[cfg(windows)]
        {
            Encoding::Windows1252
        }
        #[cfg(not(windows))]
        {
            Encoding::Utf8
        }
    }

    /// Parse byte content into a concrete type via serde.
    ///
    /// # Errors
    ///
    /// Returns a jomini error if the content cannot be deserialized.
    fn parse_content<T>(&self, bytes: &[u8]) -> Result<T, jomini::Error>
    where
        T: serde::de::DeserializeOwned,
    {
        let tape = jomini::TextTape::from_slice(bytes)?;
        let encoding = self.get_encoding();
        match encoding {
            Encoding::Windows1252 => tape.windows1252_reader().deserialize(),
            Encoding::Utf8 => tape.utf8_reader().deserialize(),
        }
    }

    /// Read a file and parse its contents into a concrete type.
    ///
    /// Returns an empty vec if the file is empty.
    ///
    /// # Errors
    ///
    /// Returns an IO error if the file cannot be read, or a jomini error if
    /// the content cannot be deserialized.
    fn parse_file<T>(&self, path: &Path) -> Result<Vec<T>, Box<dyn std::error::Error>>
    where
        T: serde::de::DeserializeOwned + Default,
    {
        let bytes = std::fs::read(path)?;
        if bytes.is_empty() {
            return Ok(vec![T::default()]);
        }
        let data: T = self.parse_content(&bytes)?;
        Ok(vec![data])
    }

    /// Collect all files with a given extension from a directory (non-recursive).
    ///
    /// The extension should be provided without a leading dot (e.g., `"txt"`).
    /// Files are returned in sorted order.
    ///
    /// # Errors
    ///
    /// Returns an IO error if the directory cannot be read.
    fn collect_files(&self, dir: &Path, extension: &str) -> io::Result<Vec<PathBuf>> {
        let mut files = Vec::new();

        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() && path.extension().is_some_and(|ext| ext == extension) {
                files.push(path);
            }
        }

        files.sort();
        Ok(files)
    }

    /// Collect all files with a given extension from a directory
    /// (non-recursive, sorted), then read and parse each one.
    ///
    /// Returns a map of file stem → parsed data.
    ///
    /// # Errors
    ///
    /// Returns an IO error if the directory cannot be read, or a jomini
    /// error if any file cannot be parsed.
    fn parse_directory<T>(
        &self,
        dir: &Path,
        extension: &str,
    ) -> Result<HashMap<String, Vec<T>>, Box<dyn std::error::Error>>
    where
        T: serde::de::DeserializeOwned + Default,
    {
        let files = self.collect_files(dir, extension)?;
        let mut results = HashMap::new();
        for file in files {
            let stem = file.file_stem().unwrap().to_string_lossy().to_string();
            let data = self.parse_file::<T>(&file)?;
            results.insert(stem, data);
        }
        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use encoding_rs;

    struct TestWindows1252Parser;

    impl TestWindows1252Parser {
        fn new() -> Self {
            TestWindows1252Parser
        }
    }

    impl Parser for TestWindows1252Parser {
        fn get_encoding(&self) -> Encoding {
            Encoding::Windows1252
        }
    }

    struct TestUtf8Parser;

    impl TestUtf8Parser {
        fn new() -> Self {
            TestUtf8Parser
        }
    }

    impl Parser for TestUtf8Parser {
        fn get_encoding(&self) -> Encoding {
            Encoding::Utf8
        }
    }

    #[test]
    fn test_parser_has_encoding() {
        let parser = TestWindows1252Parser::new();
        // just check we get something concrete without panicking
        let _ = parser.get_encoding();
    }

    #[test]
    fn test_parse_content_windows1252() {
        use serde::Deserialize;

        #[derive(Deserialize, Debug, PartialEq)]
        struct Config {
            name: String,
            count: u32,
        }

        let parser = TestWindows1252Parser::new();
        let (data, _, _) = encoding_rs::WINDOWS_1252.encode("name = test count = 42");
        let config: Config = parser.parse_content(&data).unwrap();
        assert_eq!(config, Config { name: "test".into(), count: 42 });
    }

    #[test]
    fn test_parse_content_utf8() {
        use serde::Deserialize;

        #[derive(Deserialize, Debug, PartialEq)]
        struct Simple {
            value: String,
        }

        let parser = TestUtf8Parser::new();
        let (data, _, _) = encoding_rs::UTF_8.encode("value = café");
        let result: Simple = parser.parse_content(&data).unwrap();
        assert_eq!(result, Simple { value: "café".into() });
    }

    #[test]
    fn test_parse_file() {
        use serde::Deserialize;

        #[derive(Default, Deserialize, Debug, PartialEq)]
        struct Simple {
            value: String,
        }

        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test.txt");
        std::fs::write(&path, b"value = hello").unwrap();

        let parser = TestWindows1252Parser::new();
        let result = parser.parse_file::<Simple>(&path).unwrap();
        assert_eq!(result, vec![Simple { value: "hello".into() }]);
    }

    mod collect_files_tests {
        use super::*;
        use std::io::Write;

        fn create_temp_dir() -> (tempfile::TempDir, PathBuf) {
            let dir = tempfile::tempdir().expect("failed to create temp dir");
            let path = dir.path().to_path_buf();
            (dir, path)
        }

        fn create_file(dir: &Path, name: &str, content: &[u8]) -> PathBuf {
            let path = dir.join(name);
            let mut file = fs::File::create(&path).expect("failed to create file");
            file.write_all(content).expect("failed to write content");
            path
        }

        #[test]
        fn test_collect_files_finds_matching() {
            let (_keep, dir) = create_temp_dir();
            create_file(&dir, "test.txt", b"hello");
            create_file(&dir, "other.bin", b"binary");
            create_file(&dir, "data.txt", b"data");
            let parser = TestWindows1252Parser::new();

            let files = parser.collect_files(&dir, "txt").unwrap();

            assert_eq!(files.len(), 2);
            assert!(files.iter().all(|p| p.extension().unwrap() == "txt"));
        }

        #[test]
        fn test_collect_files_no_match() {
            let (_keep, dir) = create_temp_dir();
            create_file(&dir, "data.bin", b"data");
            let parser = TestWindows1252Parser::new();

            let files = parser.collect_files(&dir, "txt").unwrap();

            assert!(files.is_empty());
        }
    }

    #[test]
    fn test_parse_directory() {
        use serde::Deserialize;

        #[derive(Default, Deserialize, Debug, PartialEq)]
        struct Config {
            name: String,
        }

        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("a.txt"), b"name = foo").unwrap();
        std::fs::write(dir.path().join("b.txt"), b"name = bar").unwrap();
        // wrong extension — should be skipped
        std::fs::write(dir.path().join("c.bin"), b"name = baz").unwrap();

        let parser = TestWindows1252Parser::new();
        let result = parser.parse_directory::<Config>(dir.path(), "txt").unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result["a"][0].name, "foo");
        assert_eq!(result["b"][0].name, "bar");
    }
}
