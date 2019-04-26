use std::path::Path;
use std::fs;
use std::env;
use fs_extra::dir;
use fs_extra::copy_items;
use walkdir::WalkDir;

const FXITURES_DIR: &str = "./fixtures";

fn main() {
    // let paths = fs::read_dir(FXITURES_DIR).unwrap();
    // let mut from_paths = Vec::new();
    // let target_dir_path = env::var("OUT_DIR").unwrap();
    // let mut options = dir::CopyOptions::new();
    // options.copy_inside = true;

    // for entry in WalkDir::new(FXITURES_DIR) {
    //     let entry = entry.unwrap();
    //     let path = entry.path().display().to_string();
    //     let clean_path = path.replace("fixtures/", &target_dir_path);
    //     from_paths.push(clean_path);
    // }

    // copy_items(&from_paths, target_dir_path, &options);
}

fn copy<S: AsRef<std::ffi::OsStr> + ?Sized, P: Copy + AsRef<Path>>(target_dir_path: &S, file_name: P) {
    fs::copy(file_name, Path::new(&target_dir_path).join("../../..").join(file_name)).unwrap();
}