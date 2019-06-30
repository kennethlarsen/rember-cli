use clap::{App, ArgMatches};


pub trait Command {
    fn config<'a, 'b>() -> App<'a, 'b>;
    fn execute<'a>(args: &ArgMatches<'a>);
}
