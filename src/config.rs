pub use config::Config;

#[derive(Debug)]
#[allow(unused)]
struct Settings {
    db_file: String,
    project_dirs: [String],
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
    // how do I set up dummy config file? this assumes the existence of a
    // file with certain contents
    let config = get_config();
    let db_file: String = config.get("db_file").unwrap();
    let project_dirs = config.get_array("project_dirs").unwrap();
    assert_eq!(db_file, "projects.db3");
    assert_eq!(project_dirs.len(), 1);
    for dir in project_dirs.iter() {
        // is there some way to do this without ownership?
        let d = dir.to_owned().into_string().unwrap();
        assert_eq!(d, "/home/lloyd/Devel/Personal");
    }
}
