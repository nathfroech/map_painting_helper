use crate::settings::Settings;

mod settings;

#[expect(dead_code)]
pub struct App {
    settings: Settings,
}

impl App {
    #[must_use]
    pub const fn new(settings: Settings) -> Self {
        Self { settings }
    }

    /// Run the parsing process.
    ///
    /// # Errors
    ///
    /// TBD
    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
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

        #[test]
        fn test_app_new() {
            let settings = Settings {
                eu4_settings: Some(EU4Settings { game_path: Some("C:\\Games\\EU4".into()) }),
            };
            let app = App::new(settings);
            assert!(app.settings.eu4_settings.is_some());
        }

        #[test]
        fn test_app_run() {
            let settings = Settings { eu4_settings: None };
            let app = App::new(settings);
            assert!(app.run().is_ok());
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
