use crate::config::Config;
use std::path::Path;
use std::path::PathBuf;

pub async fn file_path_validator(config: PathBuf) {
    // load the config
    let loader = Config::load(&config.as_path()).unwrap();
    let files = loader.sources.files;

    // An array to hold non-existence paths
    let mut no_existent_path = Vec::new();

    // iterate over the path and check if they exists
    for path in files.logs.iter() {
        if !Path::new(&path["path"]).exists() {
            no_existent_path.push(&path["path"]);
        }
    }

    if !no_existent_path.is_empty() {
        log::error!("Check Failed!");
        log::error!("{:?}", no_existent_path);
    } else {
        log::info!("Check Ok!");
    }
}
