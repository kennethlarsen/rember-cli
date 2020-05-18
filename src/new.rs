use super::fixtures::*;
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
    generate_empty_app_files(name);
    generate_app_files(name);
    update_values_in_files("<%= name %>", name, &format!("{}/package.json", name))
        .expect("Couldn't replace placeholders in generated files.");

    update_values_in_files("<%= name %>", name, &format!("{}/app/index.html", name))
        .expect("Couldn't replace placeholders in generated files.");
    update_values_in_files(
        "<%= namespace %>",
        name,
        &format!("{}/app/index.html", name),
    )
    .expect("Couldn't replace placeholders in generated files.");

    update_values_in_files(
        "<%= modulePrefix %>",
        name,
        &format!("{}/config/environment.js", name),
    )
    .expect("Couldn't replace placeholders in generated files.");
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

fn generate_empty_app_files(name: &str) {
    let file_names = vec![
        "app/styles/app.css",
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

fn generate_app_files(name: &str) {
    // Here we write all the files with fixture data from src/fixtures.
    // It feels very dirty, but it works.
    write_all(
        format!("{}/app/templates/application.hbs", name),
        templates::get_application_hbs(),
    );
    write_all(format!("{}/app/app.js", name), js::get_app_js());
    write_all(
        format!("{}/app/index.html", name),
        templates::get_index_html(),
    );
    write_all(format!("{}/app/resolver.js", name), js::get_resolver_js());
    write_all(format!("{}/app/router.js", name), js::get_router_js());
    write_all(
        format!("{}/config/environment.js", name),
        js::get_environment_js(),
    );
    write_all(
        format!("{}/config/optional-features.json", name),
        json::get_optional_features(),
    );
    write_all(format!("{}/config/targets.js", name), js::get_target_js());
    write_all(format!("{}/public/robots.txt", name), txt::get_robots_txt());

    // TODO Move these to the fixtures folder
    write_all(
        format!("{}/tests/index.html", name),
        html::get_test_index_html(),
    );

    write_all(
        format!("{}/tests/test-helper.js", name),
        js::get_test_helper_js(),
    );

    write_all(
        format!("{}/.editorconfig", name),
        config::get_editor_config(),
    );

    write_all(format!("{}/.ember-cli", name), config::get_ember_cli());

    write_all(
        format!("{}/.eslintignore", name),
        config::get_eslint_ignore(),
    );

    write_all(format!("{}/.eslintrc.js", name), config::get_eslintrc_js());

    write_all(
        format!("{}/.template-lintrc.js", name),
        config::get_template_lintrc_js(),
    );

    write_all(
        format!("{}/.watchmanconfig", name),
        config::get_watchmanconfig(),
    );

    write_all(
        format!("{}/ember-cli-build.js", name),
        config::get_ember_cli_build(),
    );

    write_all(format!("{}/gitignore", name), config::get_gitignore());

    write_all(format!("{}/package.json", name), json::get_package_json());

    write_all(format!("{}/README.md", name), config::get_readme_md());

    write_all(format!("{}/testem.js", name), js::get_testem_js());
}
