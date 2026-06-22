use crate::settings::Settings;
use std::fs;
use std::io::Write;
extern crate env_settings_derive;
extern crate jomini;
extern crate serde;
extern crate serde_json;

mod common;
mod eu4;
mod settings;

#[allow(dead_code)]
pub struct App {
    settings: Settings,
}

impl App {
    #[must_use]
    pub const fn new(settings: Settings) -> Self {
        Self { settings }
    }

    /// Executes the main logic for parsing Europa Universalis IV data, serializing it into JSON,
    /// and saving it to a file. This function requires the settings to be correctly configured
    /// with the game path and an optional output path.
    ///
    /// # Returns
    /// * `Result<(), Box<dyn std::error::Error>>` - Returns `Ok(())` if the operation completes successfully,
    ///   or an error if any stage of the operation fails.
    ///
    /// # Errors
    /// This function will return an error in the following cases:
    /// - The game path is not set in the settings.
    /// - Any error occurs while creating the `parsed_data` directory.
    /// - Serialization of the parsed data into JSON fails.
    /// - Writing to the output file fails.
    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Running parser for Europa Universalis IV");
        let Some(game_path) = self.settings.eu4_settings.as_ref().and_then(|s| s.game_path.clone())
        else {
            println!("EU4 game path not set");
            return Err("EU4 game path not set".into());
        };
        let mut parser = eu4::parser::EU4Parser::new(game_path);
        parser.parse_country_tags()?;

        // Create parsed_data directory if it doesn't exist
        fs::create_dir_all("parsed_data")?;

        // Serialize and write parser.store to eu4.json
        let json = serde_json::to_string_pretty(&parser.store)?;
        let output_path = self
            .settings
            .eu4_settings
            .as_ref()
            .and_then(|s| s.output_path.clone())
            .unwrap_or_else(|| std::path::PathBuf::from("../parsed_data/eu4.json"));
        let mut file = fs::File::create(output_path)?;
        file.write_all(json.as_bytes())?;

        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = Settings::from_env();
    let app = App::new(settings);

    app.run()
}

#[cfg(test)]
mod tests {
    use super::*;

    mod app_tests {
        use super::*;
        use crate::settings::EU4Settings;

        fn make_test_settings() -> Settings {
            Settings {
                eu4_settings: Some(EU4Settings {
                    game_path: Some("C:\\Games\\EU4".into()),
                    output_path: None,
                }),
            }
        }

        #[test]
        fn test_app_new() {
            let settings = make_test_settings();
            let app = App::new(settings);
            assert!(app.settings.eu4_settings.is_some());
        }

        #[test]
        fn test_app_run() {
            let settings = make_test_settings();
            let app = App::new(settings);
            assert!(app.run().is_ok());
        }

        #[test]
        fn test_app_run_fails() {
            let settings =
                Settings { eu4_settings: Some(EU4Settings { game_path: None, output_path: None }) };
            let app = App::new(settings);
            assert!(app.run().is_err());
        }
    }

    mod main_tests {
        use crate::main;

        #[test]
        fn test_main() {
            let result = main();
            assert!(result.is_ok());
        }
    }
}
