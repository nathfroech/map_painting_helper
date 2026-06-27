pub mod common;
pub mod eu4;
pub mod settings;

use std::fs;
use std::io::Write;
use std::path::PathBuf;

/// Parses EU4 data and returns it as a JSON string.
///
/// # Errors
///
/// Returns an error if file reading or JSON serialization fails.
pub fn parse_eu4(game_path: PathBuf) -> Result<String, Box<dyn std::error::Error>> {
    println!("Running parser for Europa Universalis IV");
    let mut parser = eu4::parser::EU4Parser::new(game_path);
    parser.parse_country_tags()?;
    let json = serde_json::to_string(&parser.store)?;
    Ok(json)
}

#[allow(dead_code)]
pub struct App {
    settings: settings::Settings,
}

impl App {
    #[must_use]
    pub const fn new(settings: settings::Settings) -> Self {
        Self { settings }
    }

    /// Executes the main parsing logic and writes JSON output to file.
    ///
    /// # Errors
    ///
    /// Returns an error if EU4 settings are missing, directory creation fails,
    /// serialization fails, or file writing fails.
    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let eu4_settings = self
            .settings
            .eu4_settings
            .as_ref()
            .ok_or("Some of the required EU4 environment variables are missing, aborting.")?;

        let json = parse_eu4(eu4_settings.game_path.clone())?;

        if let Some(parent) =
            eu4_settings.output_path.parent().filter(|p| !p.as_os_str().is_empty())
        {
            fs::create_dir_all(parent)?;
        }
        let mut file = fs::File::create(&eu4_settings.output_path)?;
        file.write_all(json.as_bytes())?;

        Ok(())
    }
}

#[cfg(feature = "pyo3")]
mod pyo3_bindings {
    use pyo3::prelude::*;
    use std::path::PathBuf;

    #[pyfunction]
    fn parse_eu4(game_path: String) -> PyResult<String> {
        super::parse_eu4(PathBuf::from(game_path))
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
    }

    #[pymodule]
    fn parser(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
        m.add_function(wrap_pyfunction!(parse_eu4, m)?)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod app_tests {
        use super::*;
        use crate::settings::EU4Settings;

        fn make_test_settings(output_path: PathBuf) -> settings::Settings {
            settings::Settings {
                eu4_settings: Some(EU4Settings { game_path: "C:\\Games\\EU4".into(), output_path }),
            }
        }

        #[test]
        fn test_app_new() {
            let temp_dir = tempfile::tempdir().unwrap();
            let output_path = temp_dir.path().join("test_eu4.json");
            let settings = make_test_settings(output_path);
            let app = App::new(settings);

            assert_eq!(
                app.settings.eu4_settings.as_ref().unwrap().game_path,
                PathBuf::from("C:\\Games\\EU4")
            );
        }

        #[test]
        fn test_app_run() {
            let temp_dir = tempfile::tempdir().unwrap();
            let output_path = temp_dir.path().join("test_eu4.json");
            let settings = make_test_settings(output_path);
            let app = App::new(settings);

            assert!(app.run().is_ok());
        }

        #[test]
        fn test_app_run_with_output_filename_only() {
            let settings = make_test_settings(PathBuf::from("test_eu4.json"));
            let app = App::new(settings);

            assert!(app.run().is_ok());

            let _ = fs::remove_file("test_eu4.json");
        }

        #[test]
        fn test_app_run_fails_on_missing_eu4_settings() {
            let settings = settings::Settings { eu4_settings: None };
            let app = App::new(settings);

            assert!(app.run().is_err());
        }

        #[test]
        fn test_parse_eu4_returns_valid_json() {
            let temp_dir = tempfile::tempdir().unwrap();
            let json = parse_eu4(temp_dir.path().to_path_buf()).unwrap();

            let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
            assert!(parsed.is_object());
        }

        #[test]
        fn test_parse_eu4_empty_store_is_valid_json() {
            let temp_dir = tempfile::tempdir().unwrap();
            let json = parse_eu4(temp_dir.path().to_path_buf()).unwrap();
            let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

            assert!(parsed.is_object());
            assert_eq!(parsed.as_object().unwrap().len(), 0);
        }
    }
}
