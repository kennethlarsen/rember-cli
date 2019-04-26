use clap::{clap_app};
use rember::new::{generate_new_application};

fn main() {
    let matches = clap_app!(myapp =>
        (version: "0.1.5")
        (author: "Kenneth Larsen <hello@kennethlarsen.org>")
        (about: "Rust clone of Ember CLI")
        (@subcommand new =>
            (about: "generates a new Ember application")
            (@arg name: +required "The name of your application")
        )
    ).get_matches();

    if let Some(matches) = matches.subcommand_matches("new") {
        if matches.is_present("name") {
            let name = matches.value_of("name").unwrap();
            generate_new_application(name);
        } else {
            println!("Error");
        }
    }
}