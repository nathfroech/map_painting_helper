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
pub(crate) mod test_utils {
    use std::sync::{Mutex, MutexGuard, OnceLock};

    static ENV_MUTEX: OnceLock<Mutex<()>> = OnceLock::new();

    pub(crate) struct EnvVarGuard {
        _lock: MutexGuard<'static, ()>,
        game_path_prev: Option<std::ffi::OsString>,
        output_path_prev: Option<std::ffi::OsString>,
    }

    impl EnvVarGuard {
        pub(crate) fn new() -> Self {
            let lock = ENV_MUTEX.get_or_init(|| Mutex::new(())).lock().unwrap();
            let game_path_prev = std::env::var_os("EU4_GAME_PATH");
            let output_path_prev = std::env::var_os("EU4_OUTPUT_PATH");

            Self { _lock: lock, game_path_prev, output_path_prev }
        }

        pub(crate) fn set(var: &str, value: impl AsRef<std::ffi::OsStr>) {
            unsafe {
                std::env::set_var(var, value);
            }
        }

        pub(crate) fn remove(var: &str) {
            unsafe {
                std::env::remove_var(var);
            }
        }
    }

    impl Drop for EnvVarGuard {
        fn drop(&mut self) {
            match &self.game_path_prev {
                Some(value) => Self::set("EU4_GAME_PATH", value),
                None => Self::remove("EU4_GAME_PATH"),
            }

            match &self.output_path_prev {
                Some(value) => Self::set("EU4_OUTPUT_PATH", value),
                None => Self::remove("EU4_OUTPUT_PATH"),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::test_utils::EnvVarGuard;
    use super::*;

    #[test]
    fn test_settings_from_env() {
        let _guard = EnvVarGuard::new();

        let expected_path = "C:\\Games\\EU4";
        let expected_output = "../parsed_data/test_eu4.json";

        EnvVarGuard::set("EU4_GAME_PATH", expected_path);
        EnvVarGuard::set("EU4_OUTPUT_PATH", expected_output);

        let settings = Settings::from_env();
        let eu4 = settings.eu4_settings.expect("EU4 settings should be Some");

        assert_eq!(eu4.game_path, PathBuf::from(expected_path));
        assert_eq!(eu4.output_path, PathBuf::from(expected_output));
    }

    #[test]
    fn test_missing_settings() {
        let _guard = EnvVarGuard::new();

        EnvVarGuard::remove("EU4_GAME_PATH");
        EnvVarGuard::remove("EU4_OUTPUT_PATH");

        let settings = Settings::from_env();

        let eu4 = settings.eu4_settings;
        assert!(eu4.is_none());
    }
}
