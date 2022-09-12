use clap::{Parser, ValueEnum};

fn main() -> retool::Result<()> {
    let app = App::parse();

    let converter = app.kind.converter();

    retool::convert_file(converter, &app.input, &app.output)
}

#[derive(Parser)]
#[clap(version, about, long_about = None)]
struct App {
    /// Kind of resource to convert
    #[clap(arg_enum, value_parser, value_name = "kind")]
    kind: Kind,

    /// Path to the input JSON file
    #[clap(value_parser)]
    input: String,

    /// Destination path for output JSON file
    #[clap(value_parser)]
    output: String,
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
