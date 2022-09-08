fn main() {
    let input_path = "./data/input/backgrounds.json";
    let output_path = "./data/output/backgrounds.json";

    retool::convert_file(input_path, output_path);
}
