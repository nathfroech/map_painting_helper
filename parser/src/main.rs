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
    /// - Any error occurs while creating the `parsed_data` directory.
    /// - Serialization of the parsed data into JSON fails.
    /// - Writing to the output file fails.
    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: At some point there will be a possibility to select between games - but for now EU4 runs by default
        let eu4_settings = self
            .settings
            .eu4_settings
            .as_ref()
            .ok_or("Some of the required EU4 environment variables are missing, aborting.")?;

        println!("Running parser for Europa Universalis IV");
        let mut parser = eu4::parser::EU4Parser::new(eu4_settings.game_path.clone());
        parser.parse_country_tags()?;

        if let Some(parent) = eu4_settings.output_path.parent() {
            fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_string_pretty(&parser.store)?;
        let mut file = fs::File::create(&eu4_settings.output_path)?;
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
        use std::path::PathBuf;

        fn make_test_settings() -> Settings {
            Settings {
                eu4_settings: Some(EU4Settings {
                    game_path: "C:\\Games\\EU4".into(),
                    output_path: "..\\parsed_data\\test_eu4.json".into(),
                }),
            }
        }

        #[test]
        fn test_app_new() {
            let settings = make_test_settings();
            let app = App::new(settings);
            assert_eq!(
                app.settings.eu4_settings.as_ref().unwrap().game_path,
                PathBuf::from("C:\\Games\\EU4")
            );
        }

        #[test]
        fn test_app_run() {
            let settings = make_test_settings();
            let app = App::new(settings);
            assert!(app.run().is_ok());
        }

        #[test]
        fn test_app_run_fails_on_missing_eu4_settings() {
            let settings = Settings { eu4_settings: None };
            let app = App::new(settings);
            assert!(app.run().is_err());
        }
    }

    mod main_tests {
        use crate::main;

        #[test]
        fn test_main() {
            unsafe {
                std::env::set_var("EU4_GAME_PATH", "C:\\Games\\EU4");
                std::env::set_var("EU4_OUTPUT_PATH", "../parsed_data/test_eu4.json");
            };

            let result = main();

            assert!(result.is_ok());
        }
    }
}
