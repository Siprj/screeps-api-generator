use std::process::exit;

use isahc::{Body, ReadResponseExt};
use log::{error, info, trace};
use scraper::ElementRef;

use simplelog::*;
use color_eyre::eyre::Result;
use clap::Parser;
use http::{Response, uri};
use scraper::Html;
use scraper::Selector;


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

#[derive(Debug)]
struct JSTypeMap {
    key_type: Box<JSType>,
    value_type: Box<JSType>,
}

#[derive(Debug)]
struct JSObjectPair {
    key: String, // This is the access key to given value
    value_type: JSType,
}

#[derive(Debug)]
struct JSTypeObject {
    pairs: Vec<JSObjectPair>,
}

#[derive(Debug)]
enum JSType {
    Map(JSTypeMap),
    Object(JSTypeObject),
    JSNumber,
    JSString,
    KnownObject(String),
}

#[derive(Debug)]
struct ParsedPropertyDescription {
    name: String,
    doc: String,
    property_type: String,
}

#[derive(Debug)]
struct ParsedMethodDescription {
    name: String,
    doc: String,
    method_type: String,
}

#[derive(Debug)]
struct ParsedApiModule {
    name: String,
    doc: String,
    methods: Vec<ParsedMethodDescription>,
    properties: Vec<ParsedPropertyDescription>,
}

impl ParsedApiModule {
    fn new(name: String, doc: String) -> Self {
        ParsedApiModule {
            name,
            doc,
            methods: Vec::new(),
            properties: Vec::new(),
        }
    }
}

fn parse_api(document: Html) -> Result<ParsedApiModule> {

    // Select the div which contains all the documentation and check if it exists.
    // The selector (selected element) is iterator so we call `next()` to check
    // existence of the element.
    info!("Trying to get \"root\" <div>.");
    let selector = Selector::parse("div.api-content.content").expect("Can't parse selector for the \"root\" <div>!");
    let root_selection: Vec<ElementRef> = document.select(&selector).collect();

    trace!("\"root\" selection has {:?} elements", root_selection.len());

    let root = root_selection.first().expect("\"root\" <div> is missing!");

    trace!("\"root\" getting child elements and processing them.");

    // Get first child (header) to fill first
    let mut elements = root.children();
    let header_element = elements.next().expect("Missing firs documentation elements! \
        Should be first header; used to be the `Game` object.");

    // Check the first child is <h1>
    if scraper::ElementRef::wrap(header_element).expect("asdf").value().name() == "h1" {
        info!("The first header!");
    }

    Ok(ParsedApiModule {
        name: "kwa".to_string(),
        doc: "bla".to_string(),
        methods: vec!(),
        properties:  vec!(),
    })
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

    let document = Html::parse_document(&text);
    let api_spec = parse_api(document);
    info!("api_spec: {:?}", api_spec);

    Ok(())
}
