use std::{fmt::Display, path::PathBuf};

use anyhow::Context;
use clap::{Parser, ValueEnum};

fn main() -> anyhow::Result<()> {
    let app = App::parse();

    app.kind
        .converter()
        .convert_file(&app.source, &app.target)
        .with_context(|| format!("Failed to retool {}", app.kind))
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
    Feat,
    Race,
}

impl Kind {
    fn converter(&self) -> retool::Converter {
        match self {
            Kind::Dummy => retool::Converter::Dummy,
            Kind::Feat => retool::Converter::Feat,
            Kind::Race => retool::Converter::Race,
        }
    }
}

impl Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Kind::Dummy => "dummy",
            Kind::Feat => "feat",
            Kind::Race => "race",
        })
    }
}
