use env_settings_derive::EnvSettings;
use std::path::PathBuf;

#[derive(EnvSettings, Debug, Clone)]
#[env_settings(delay)]
pub struct EU4Settings {
    #[env_settings(variable = "EU4_GAME_PATH")]
    pub game_path: PathBuf,

    #[env_settings(variable = "EU4_OUTPUT_PATH")]
    pub output_path: PathBuf,
}

pub struct Settings {
    pub eu4_settings: Option<EU4Settings>,
}

impl Settings {
    pub fn from_env() -> Self {
        let eu4_settings = EU4Settings::from_env().ok();
        if eu4_settings.is_none() {
            eprintln!("Some of the required EU4 environment variables are missing");
        }
        Self { eu4_settings }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_settings_from_env() {
        let expected_path = "C:\\Games\\EU4";
        let expected_output = "../parsed_data/test_eu4.json";
        unsafe {
            std::env::set_var("EU4_GAME_PATH", expected_path);
            std::env::set_var("EU4_OUTPUT_PATH", expected_output);
        };
        let settings = Settings::from_env();
        let eu4 = settings.eu4_settings.expect("EU4 settings should be Some");
        assert_eq!(eu4.game_path, PathBuf::from(expected_path));
        assert_eq!(eu4.output_path, PathBuf::from(expected_output));
    }

    #[test]
    fn test_missing_settings() {
        unsafe {
            std::env::remove_var("EU4_GAME_PATH");
            std::env::remove_var("EU4_OUTPUT_PATH");
        }
        let settings = Settings::from_env();

        let eu4 = settings.eu4_settings;
        assert!(eu4.is_none());
    }
}
