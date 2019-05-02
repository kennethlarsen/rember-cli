use std::path::Path;
use std::fs;
use std::env;
use fs_extra::dir;
use fs_extra::copy_items;
use walkdir::WalkDir;

const FXITURES_DIR: &str = "./fixtures";

fn main() {
    let mut from_paths = Vec::new();
    let target_dir_path = env::var("OUT_DIR").unwrap();
    let mut options = dir::CopyOptions::new();
    options.copy_inside = true;
    options.overwrite = true;

    if Path::new(&format!("{}/fixtures", target_dir_path)).exists() {
        println!("It exists");
        fs::remove_dir_all(&format!("{}/fixtures", target_dir_path));
    }

    fs::create_dir_all(&format!("{}/fixtures", target_dir_path));

    for entry in WalkDir::new(FXITURES_DIR) {
        let entry = entry.unwrap();
        let path = entry.path().display().to_string();
        let clean_path = path.replace("./fixtures", &format!("{}/fixtures", &target_dir_path));
        println!("{}", path);
        from_paths.push(path);
    }

    copy_items(&from_paths, format!("{}/fixtures", target_dir_path), &options).expect("Couldn't copy files in build");
}