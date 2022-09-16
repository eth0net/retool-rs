use std::{path::PathBuf, process};

use clap::{Parser, ValueEnum};

fn main() {
    let app = App::parse();

    let converter = app.kind.converter();

    if let Err(error) = retool::convert_file(converter, &app.source, &app.target) {
        eprintln!("Retool encountered an error: {}", error);
        process::exit(1);
    };
}

#[derive(Parser)]
#[clap(version, about, long_about = None)]
struct App {
    /// Kind of resource to convert
    #[clap(arg_enum, value_parser, value_name = "kind")]
    kind: Kind,

    /// Path to the input JSON file
    #[clap(value_parser, value_name = "source")]
    source: PathBuf,

    /// Destination path for output JSON file
    #[clap(value_parser, value_name = "target")]
    target: PathBuf,
}

#[derive(Clone, ValueEnum)]
enum Kind {
    Dummy,
}

impl Kind {
    fn converter(&self) -> retool::Converter {
        match self {
            Kind::Dummy => retool::Converter::Dummy,
        }
    }
}
