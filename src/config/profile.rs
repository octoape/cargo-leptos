use core::fmt;

#[derive(Debug)]
pub enum Profile {
    Debug,
    Release,
    Named(String),
}

impl fmt::Display for Profile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Debug => write!(f, "debug"),
            Self::Release => write!(f, "release"),
            Self::Named(name) => write!(f, "{name}"),
        }
    }
}

impl Profile {
    pub fn new(is_release: bool, release: &Option<String>, debug: &Option<String>) -> Self {
        if is_release {
            if let Some(release) = release {
                Self::Named(release.clone())
            } else {
                Self::Release
            }
        } else if let Some(debug) = debug {
            Self::Named(debug.clone())
        } else {
            Self::Debug
        }
    }

    /// The `target` subdirectory to which Cargo writes this profile's artifacts to.
    ///
    /// This is usually the profile name, but cargo's built-in `dev` and `test` profiles build
    /// into `target/debug`, and `release` and `bench` build into `target/release`.
    pub fn dir_name(&self) -> &str {
        match self {
            Self::Debug => "debug",
            Self::Release => "release",
            Self::Named(name) => match name.as_str() {
                "dev" | "test" => "debug",
                "bench" => "release",
                name => name,
            },
        }
    }

    pub fn add_to_args(&self, args: &mut Vec<String>) {
        match self {
            Self::Debug => {}
            Self::Release => {
                args.push("--release".to_string());
            }
            Self::Named(name) => {
                args.push(format!("--profile={name}"));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn built_in_profiles_use_cargo_dir_names() {
        assert_eq!(Profile::Debug.dir_name(), "debug");
        assert_eq!(Profile::Release.dir_name(), "release");
        assert_eq!(Profile::Named("dev".into()).dir_name(), "debug");
        assert_eq!(Profile::Named("test".into()).dir_name(), "debug");
        assert_eq!(Profile::Named("release".into()).dir_name(), "release");
        assert_eq!(Profile::Named("bench".into()).dir_name(), "release");
    }

    #[test]
    fn custom_profiles_use_their_own_dir_name() {
        assert_eq!(Profile::Named("leptos-dev".into()).dir_name(), "leptos-dev");
        assert_eq!(
            Profile::Named("wasm-release".into()).dir_name(),
            "wasm-release"
        );
    }
}
