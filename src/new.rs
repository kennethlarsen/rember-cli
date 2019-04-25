use std::fs;
use walkdir::WalkDir;
use fs_extra::dir;
use fs_extra::copy_items;

pub fn generate_new_application(name: &str) -> Result<(), fs_extra::error::Error> {
    let app_path = format!("{}", name);
    let mut options = dir::CopyOptions::new();
    let mut from_paths = Vec::new();

    options.copy_inside = true;

    for entry in WalkDir::new("fixtures") {
        let entry = entry.unwrap();
        let path = entry.path().display().to_string();
        let clean_path = path.replace("fixtures/", &format!("{}/", name));
        println!("{}", clean_path);
        from_paths.push(clean_path);
    }

    copy_items(&from_paths, format!("{}", name), &options);
    Ok(())
}
