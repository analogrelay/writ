use clap::{App, Arg, SubCommand};

pub fn build_subcommands() -> Vec<App<'static, 'static>> {
    vec![SubCommand::with_name("show-ref")
        .about("List references in a local repository")
        .arg(Arg::with_name("pattern").index(1).multiple(true))]
}