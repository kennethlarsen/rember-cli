use super::utils::{create_progress_bar, update_values_in_files};
use fs_extra::dir::create;
use fs_extra::file::write_all;
use std::env;
use std::process::Command;

pub fn generate_new_application(
    name: &str,
    install_dependencies: bool,
) -> Result<(), fs_extra::error::Error> {
    let progress_bar = create_progress_bar(false, "🗄  Generating files...", Some(23));
    // Create main app folder
    create(format!("{}/", name), false);

    generate_app_folder_structure(name);
    generate_app_files(name);
    // update_values_in_files("<%= name %>", name, &format!("{}/package.json", name))
    //     .expect("Couldn't replace placeholders in generated files.");

    progress_bar.finish();
    // env::set_current_dir(format!("{}/", name))
    //     .expect("Couldn't find the newly generated project folder.");

    if install_dependencies {
        // npm_install();
    }

    Ok(())
}

fn npm_install() {
    let progress_bar = create_progress_bar(false, "🚚 Running npm install...", None);
    let npm_install = Command::new("npm")
        .arg("install")
        .arg("--loglevel error")
        .output()
        .expect("Failed to npm install");

    if npm_install.status.success() {
        progress_bar.finish();
    }
}

fn generate_app_folder_structure(name: &str) {
    let folder_names = vec![
        "app",
        "app/components",
        "app/controllers",
        "app/helpers",
        "app/models",
        "app/routes",
        "app/styles",
        "app/templates",
        "app/templates/components",
        "config",
        "public",
        "tests",
        "tests/helpers",
        "tests/integration",
        "tests/unit",
        "vendor",
    ];

    for folder in &folder_names {
        create(format!("{}/{}", name, folder), false);
    }
}

fn generate_app_files(name: &str) {
    let file_names = vec![
        "styles/app.css",
        "templates/components/application.hbs",
        "app.js",
        "index.html",
        "resolver.js",
        "router.js",
        "config/environment.js",
        "config/optional-features.json",
        "target.js",
        "public/robots.txt",
        "tests/index.html",
        "tests/test-helper.js",
        "editorconfig",
        ".ember-cli",
        ".eslintignore",
        ".eslintrc.js",
        ".template-lintrc.js",
        ".travis.yml",
        ".watchmanconfig",
        "ember-cli-build.js",
        "gitignore",
        "package.json",
        "README.md",
        "testem.js",
    ];

    for file in &file_names {
        write_all(format!("{}/{}", name, file), "");
    }
}
