use actix_web::{fs, App, server};
use std::process::Command;

pub fn start() {

  Command::new("ember")
        .arg("build")
        .arg("--watch")
        .spawn()
        .expect("Failed to build");

    println!("Starting server");
    App::new()
        .handler(
            "/new-project/dist",
            fs::StaticFiles::new(".")
                .unwrap()
                .show_files_listing())
        .finish();
    
}