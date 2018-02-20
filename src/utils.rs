use std::env;

/// Returns current working dir
///
/// Help for simpler use in FormField
pub fn cwd() -> String {
    env::current_dir()
        .map(|p| p.into_os_string().into_string().unwrap())
        .ok()
        .unwrap()
}

/// Returns home-dir path
//TODO: -> String
pub fn home_dir() -> Option<String> {
    env::home_dir().map(|p| p.into_os_string().into_string().unwrap())
}
