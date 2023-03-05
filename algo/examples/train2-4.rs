fn main() {
    let binding = get_input();
    let input_string = binding.trim();

    let split_input = parse_input_strings(input_string);

    let value_list: Vec<i32> = split_input
        .iter()
        .map(|string| string.parse().unwrap())
        .collect();

    let a = &value_list[0];
    let b = &value_list[1];
    let result = if calc_pair(a, b) {"Yes"} else {"No"};

    println!("{}", result);
}

fn calc_pair<'a>(a: &'a i32, b: &'a i32) -> bool {
    a % b == 0
}

fn parse_input_strings(string: &str) -> Vec<&str> {
    string.split(" ").collect()
}

fn get_input() -> String {
    let mut buf: String = "".to_string();
    std::io::stdin().read_line(&mut buf).ok();
    buf
}
