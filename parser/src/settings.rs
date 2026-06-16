use env_settings_derive::EnvSettings;
use std::path::PathBuf;

#[derive(EnvSettings, Debug)]
pub struct EU4Settings {
    #[env_settings(variable = "EU4_GAME_PATH")]
    pub game_path: Option<PathBuf>,
}

pub struct Settings {
    pub eu4_settings: Option<EU4Settings>,
}

impl Settings {
    pub fn from_env() -> Self {
        Self { eu4_settings: EU4Settings::from_env().ok() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_settings_from_env() {
        let expected_path = "C:\\Games\\EU4";
        unsafe { std::env::set_var("EU4_GAME_PATH", expected_path) };
        let settings = Settings::from_env();
        assert!(settings.eu4_settings.is_some());
    }
}
