use walkdir::WalkDir;
use fs_extra::dir;
use fs_extra::copy_items;


pub fn generate_new_application(name: &str) -> Result<(), fs_extra::error::Error> {
    let mut options = dir::CopyOptions::new();
    let mut from_paths = Vec::new();
    let root_path = format!("{}/registry/src/github.com-1ecc6299db9ec823/rember-0.1.4/fixtures/", env!("CARGO_HOME"));
    options.copy_inside = true;

    for entry in WalkDir::new(root_path) {
        let entry = entry.unwrap();
        let path = entry.path().display().to_string();
        from_paths.push(path);
    }

    copy_items(&from_paths, format!("{}/", name), &options);
    Ok(())
}
