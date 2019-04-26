use clap::{clap_app};
use rember::new::{generate_new_application};
use rember::utils::{update_values_in_files};

fn main() {
    let matches = clap_app!(myapp =>
        (version: "0.1.6")
        (author: "Kenneth Larsen <hello@kennethlarsen.org>")
        (about: "Rust clone of Ember CLI")
        (@arg MANIFEST: -m --manifest +takes_value "Manifest")
        (@subcommand new =>
            (about: "generates a new Ember application")
            (@arg name: +required "The name of your application")
        )
    ).get_matches();

    if let Some(matches) = matches.subcommand_matches("new") {
        if matches.is_present("name") {
            let name = matches.value_of("name").unwrap();
            generate_new_application(name);
            update_values_in_files("<%= name %>", name, &format!("{}/package.json", name));

        } else {
            println!("Error");
        }
    }
}
