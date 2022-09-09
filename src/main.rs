fn main() -> retool::Result<()> {
    let data_kind = "dummy";
    let input_path = "./data/input/backgrounds.json";
    let output_path = "./data/output/backgrounds.json";

    retool::convert_file(data_kind, input_path, output_path)
}
