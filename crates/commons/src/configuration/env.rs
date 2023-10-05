use std::path::PathBuf;
use strum_macros::EnumString;
use tap::TapFallible;

const RUN_ENV: &str = "RUN_ENV";
const CONFIG_DIR: &str = "CONFIG_DIR";
const CARGO_MANIFEST_DIR: &str = "CARGO_MANIFEST_DIR";
const DEFAULT_CONFIG_FOLDER: &str = "configuration";

#[derive(Debug, Default, PartialEq, Eq, EnumString, strum::Display, Clone, Copy)]
#[strum(ascii_case_insensitive)]
pub enum Environment {
    #[default]
    Development,
    Production,
    Sandbox,
}

impl Environment {
    pub fn which() -> Self {
        std::env::var(RUN_ENV)
            .tap_err(|e| {
                tracing::warn!(
                    "Failed to fetch [{}] from environment. Error: {e:?}",
                    RUN_ENV
                )
            })
            .map_or_else(
                |_| Default::default(),
                |v| {
                    v.parse()
                        .tap_err(|e| {
                            tracing::warn!("Failed to parse {} to Environment. Error: {e:?}", v)
                        })
                        .unwrap_or_default()
                },
            )
    }
    /// Config path.
    pub fn config_path() -> PathBuf {
        let mut config_path = PathBuf::new();
        let config_directory =
            std::env::var(CONFIG_DIR).unwrap_or_else(|_| DEFAULT_CONFIG_FOLDER.into());
        config_path.push(Self::workspace_path());
        config_path.push(config_directory);
        config_path
    }

    fn workspace_path() -> PathBuf {
        std::env::var(CARGO_MANIFEST_DIR).map_or_else(
            |_| PathBuf::from("."),
            |manifest_dir| {
                let mut path = PathBuf::from(manifest_dir);
                path.pop();
                path.pop();
                path
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(None, Environment::Development)]
    #[case(Some("production"), Environment::Production)]
    #[case(Some("sandbox"), Environment::Sandbox)]
    #[case(Some("invalid"), Environment::Development)]
    fn test_which(#[case] env_value: Option<&str>, #[case] environment: Environment) {
        if env_value.is_some() {
            temp_env::with_var(RUN_ENV, env_value, || {
                assert_eq!(Environment::which(), environment);
            });
        } else {
            assert_eq!(Environment::which(), environment);
        }
    }

    #[rstest]
    #[case(None, None, PathBuf::from("./configuration"))]
    #[case(Some("folder"), None, PathBuf::from("./folder"))]
    #[case(None, Some("./level1/level2"), PathBuf::from("./configuration"))]
    fn test_config_path(
        #[case] env_config_dir: Option<&str>,
        #[case] env_manifest_dir: Option<&str>,
        #[case] output: PathBuf,
    ) {
        temp_env::with_vars(
            [
                (CONFIG_DIR, env_config_dir),
                (CARGO_MANIFEST_DIR, env_manifest_dir),
            ],
            || assert_eq!(Environment::config_path(), output),
        );
    }
}
