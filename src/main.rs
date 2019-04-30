use clap::{clap_app};
use rember::new::{generate_new_application};
use rember::utils::{update_values_in_files, create_progress_bar};
use std::process::Command;
use std::env;
use std::path::Path;

fn main() {
    let matches = clap_app!(myapp =>
        (version: env!("CARGO_PKG_VERSION"))
        (author: "Kenneth Larsen <hello@kennethlarsen.org>")
        (about: "Rust clone of Ember CLI")
        (@subcommand new =>
            (about: "generates a new Ember application")
            (@arg name: +required "The name of your application")
        )
    ).get_matches();

    if let Some(matches) = matches.subcommand_matches("new") {
        if matches.is_present("name") {
            // TODO: Clean this up and move to new.rs
            let name = matches.value_of("name").unwrap();
            generate_new_application(name);
            update_values_in_files("<%= name %>", name, &format!("{}/package.json", name));
            let project_name = format!("{}", name);
            let new_app_path = Path::new(&project_name);
            env::set_current_dir(&new_app_path);

            let progress_bar = create_progress_bar(false, "🚚 Running npm install...", None);

            let npm_install = Command::new("npm")
                .arg("install")
                .arg("--loglevel error")
                .output()
                .expect("Failed to npm install");

            if npm_install.status.success() {
                progress_bar.finish();
            }
        } else {
            println!("Error");
        }
    }
}
