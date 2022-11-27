pub use config::Config;
use std::env;

#[derive(Debug)]
#[allow(unused)]
struct Settings {
    db_file: String,
    project_dirs: [String],
}

pub fn expand_home(dirname: &str) -> String {
    if dirname.starts_with("~") {
        let dir_copy = dirname.clone().to_owned();
        dir_copy.replace("~", env::var("HOME").unwrap().as_str())
    } else {
        dirname.to_owned()
    }
}

#[test]
fn test_expand_home() {
    assert_eq!(expand_home("/home/fred/"), "/home/fred/");
    // TODO: is there a mock for env::var() call?
    let expanded = expand_home("~/i/am/a/teapot");
    let expected = format!("{}/i/am/a/teapot", env::var("HOME").unwrap());
    assert_eq!(expanded, expected)
}

pub fn get_config() -> Config {
    // TODO: Handle case for configuration file not found
    Config::builder()
        // what's an error case for set_default()??
        .set_default("db_file", "./projects.db3")
        .expect("unable to get config value db_file")
        .set_default("project_dirs", "/home/lloyd/Devel")
        .expect("unable to get config value project_dirs")
        .add_source(config::File::with_name("./projects.yaml"))
        .build()
        .expect("Error reading config file")
}

#[test]
fn test_config() {
    // TODO: how do I set up dummy config file? this assumes the existence of a file with certain
    // contents
    let config = get_config();
    let db_file = expand_home(config.get::<String>("db_file").unwrap().as_str());
    let project_dirs = config.get_array("project_dirs").unwrap();
    assert_eq!(db_file, String::from("/home/lloyd/.projects.db3"));
    assert_eq!(project_dirs.len(), 2);
    assert_eq!(project_dirs[0].to_string(), "/home/lloyd/Devel");
}
