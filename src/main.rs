use clap::{clap_app};
use rember::new::{generate_new_application};
use rember::component::{generate_component, generate_component_class};

fn main() {
    let matches = clap_app!(myapp =>
        (version: env!("CARGO_PKG_VERSION"))
        (author: "Kenneth Larsen <hello@kennethlarsen.org>")
        (about: "Rust clone of Ember CLI")
        (@subcommand new =>
            (about: "generates a new Ember application")
            (@arg name: +required "The name of your application")
            (@arg skip_npm: --skip "Skips npm install when generating a new Ember application")
        )
        (@subcommand generate =>
            (about: "generates a blueprint")
            (@arg blueprint: +required "The type of blueprint")
            (@arg name: +required "Name of the file")
        )
    ).get_matches();

    if let Some(matches) = matches.subcommand_matches("new") {
        if matches.is_present("name") {
            let name = matches.value_of("name").unwrap();

            if matches.is_present("skip_npm") {
                generate_new_application(name, false).expect("Generating project failed.");
            } else {
                generate_new_application(name, true).expect("Generating project failed.");
            }

        } else {
            println!("Error");
        }
    } else if let Some(matches) = matches.subcommand_matches("generate") {
        let component_class: bool = false;
        let blueprint = matches.value_of("blueprint").unwrap();
        let name = matches.value_of("name").unwrap();

        if blueprint == "component" {
          generate_component(name, component_class);
        } else if blueprint == "component-class" {
            generate_component_class(name);
        }
    }
}
