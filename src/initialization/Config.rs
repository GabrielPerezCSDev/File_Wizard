//Use once cell so that we can compile the static object at runtime
use once_cell::sync::Lazy;

pub struct Config {
    pub app_name: &'static str,
    pub version: &'static str,
    pub os: &'static str,
}

pub static CONFIG: Lazy<Config> = Lazy::new(|| Config {
    app_name: "File Expert",
    version: "1.0",
    os: determine_os(),
});

pub fn determine_os() -> &'static str{
    if cfg!(target_os = "windows") {
        return "windows";
    } else if cfg!(target_os = "linux") {
        return "linux";
    } else if cfg!(target_os = "macos") {
        return "macos";
    } else {
        //terminate program and throw error
        return "";
    }
}