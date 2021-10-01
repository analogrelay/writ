use clap::{crate_version, App, Arg};
use log::LevelFilter;
use simplelog::{ColorChoice, ConfigBuilder, LevelPadding, TermLogger, TerminalMode};

mod commands;

pub fn run<I: Iterator<Item = String>>(args: I) -> Result<(), (i32, String)> {
    let matches = build_app().get_matches_from(args);

    let mut config = ConfigBuilder::new();
    let verbosity = matches.occurrences_of("v");
    let details_level = if verbosity > 1 {
        LevelFilter::Error
    } else {
        LevelFilter::Off
    };
    config.set_time_level(details_level);
    config.set_thread_level(LevelFilter::Off);
    config.set_target_level(details_level);
    config.set_location_level(details_level);
    config.set_level_padding(LevelPadding::Left);
    config.set_max_level(LevelFilter::Error);

    let log_level = match verbosity {
        0 => LevelFilter::Info,
        1 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    };

    TermLogger::init(
        log_level,
        config.build(),
        TerminalMode::Mixed,
        ColorChoice::Always,
    ).map_err(|e| (1, format!("failed to initialize logger: {}", e)))?;

    let (name, _args) = matches.subcommand();

    log::trace!("identified command: {}", name);

    Ok(())
}

fn build_app() -> App<'static, 'static> {
    App::new("Writ")
        .version(crate_version!())
        .author("Andrew Stanton-Nurse <contact@analogrelay.dev>")
        .about("An alternate Git client, for kicks.")
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .subcommands(commands::build_subcommands())
}