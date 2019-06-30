use clap::{App, Arg, ArgMatches, SubCommand};
use super::Command;
use dialoguer::{theme::ColorfulTheme, Checkboxes, Select};
use super::super::models::{Presets};

pub struct CreateCommand {

}

impl Command for CreateCommand {
    fn config<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name("create")
            .arg(Arg::with_name("directory")
                .short("dir")
                .takes_value(true)
                .help("Location for the new project"))
    }

    fn execute<'a>(args: &ArgMatches<'a>) {
        println!("Running create");

        let templates = &[
            "Beginner\n  -> If you are new to ember, use this!",
            "Ember App\n  -> Regular ember app wo/ welcome page",
            "Ember Addon\n  -> Regular ember addon",
            "Octane App\n  -> Ember app but with the cool new shit",
            "Octane Addon\n  -> Ember addon but with the cool new shit"
        ];

        let presetNames = ["beginner", "ember-app", "ember-addon", "octane-app", "octane-addon"];


        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Pick your flavor")
            .default(0)
            .items(&templates[..])
            .interact()
            .unwrap();

        println!("Enjoy your {} name {}!", templates[selection], Presets.selection);


        // let checkboxes = &[
        //     "Ice Cream",
        //     "Vanilla Cupcake",
        //     "Chocolate Muffin",
        //     "A Pile of sweet, sweet mustard",
        // ];
        // let selections = Checkboxes::with_theme(&ColorfulTheme::default())
        //     .with_prompt("Pick your food")
        //     .items(&checkboxes[..])
        //     .interact()
        //     .unwrap();

        // if selections.is_empty() {
        //     println!("You did not select anything :(");
        // } else {
        //     println!("You selected these things:");
        //     for selection in selections {
        //         println!("  {}", checkboxes[selection]);
        //     }
        // }
    }
}
