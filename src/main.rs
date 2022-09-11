use std::{env, process};

fn main() -> retool::Result<()> {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Failed to build config: {}", err);
        process::exit(1)
    });

    retool::convert_file(config.converter, &config.input, &config.output)
}

struct Config {
    converter: retool::Converter,
    input: String,
    output: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let kind = args[1].clone();
        let input = args[2].clone();
        let output = args[3].clone();

        let converter = match kind.as_str() {
            "dummy" => retool::Converter::Dummy,
            _ => {
                return Err("unknown kind");
            }
        };

        Ok(Config {
            converter,
            input,
            output,
        })
    }
}
