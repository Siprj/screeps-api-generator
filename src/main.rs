use std::process::exit;

use isahc::{Body, ReadResponseExt};
use log::{error, warn, info, debug, trace};
use simplelog::*;
use color_eyre::eyre::Result;
use clap::Parser;
use http::{Response, uri};


#[derive(clap::ArgEnum, Clone, Debug)]
enum CliLogLevel {
    Off,
    Trace,
    Debug,
    Info,
    Warning,
    Error,
}

fn map_log_levels(level : CliLogLevel) -> log::LevelFilter {
    match level {
        CliLogLevel::Off => log::LevelFilter::Off,
        CliLogLevel::Trace => log::LevelFilter::Trace,
        CliLogLevel::Debug => log::LevelFilter::Debug,
        CliLogLevel::Info => log::LevelFilter::Info,
        CliLogLevel::Warning => log::LevelFilter::Warn,
        CliLogLevel::Error => log::LevelFilter::Trace,
    }
}

#[derive(Parser)]
struct CliOptions {
    #[clap(long, short, arg_enum, default_value = "info")]
    log_level: CliLogLevel,

    #[clap(long, default_value = "https://docs.screeps.com/api/")]
    screeps_api_doc_url: uri::Uri,
}

fn get_text_or_fail(mut resp: Response<Body>, request_url: &uri::Uri) -> String {
    if resp.status().is_success() {
        resp.text().expect("Request to [{}] can't return text from body")
    }
    else {
        error!("Request to [{}] failed with: {}", request_url, resp.status());
        exit(1);
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let cli_options = CliOptions::parse();

    CombinedLogger::init(
        vec![
            TermLogger::new(map_log_levels(cli_options.log_level), Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
        ]
    ).unwrap();


    info!("Getting screeps API documentation from: {}", cli_options.screeps_api_doc_url);
    let resp = isahc::get(&cli_options.screeps_api_doc_url)?;
    let text = get_text_or_fail(resp, &cli_options.screeps_api_doc_url);

    info!("Data: {}", text);
    trace!("a trace example");
    debug!("deboogging");
    info!("such information");
    warn!("o_O");
    error!("boom");

    Ok(())
}
