fn main() -> retool::Result<()> {
    let data_kind = "dummy";
    let input_path = "./data/input/backgrounds.json";
    let output_path = "./data/output/backgrounds.json";

    let kind = match data_kind {
        "dummy" => retool::ConverterKind::Dummy,
        _ => panic!("Unknown kind: {}", data_kind),
    };

    retool::convert_file(kind, input_path, output_path)
}
