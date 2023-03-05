
fn main() {
    let binding = get_input();
    let input_string = binding.trim();

    let split_input = parse_input_strings(input_string);

    let value_list: Vec<i32> = split_input.iter().map(|string| string.parse().unwrap()).collect();

    let max = value_list.iter().max().unwrap();

    let num = max;
    println!("{}", num);
}

fn parse_input_strings(string: &str) -> Vec<&str> {
    string.split(" ").collect()
}

fn get_input() -> String {
    let mut buf: String = "".to_string();
    std::io::stdin().read_line(&mut buf).ok();
    buf
}