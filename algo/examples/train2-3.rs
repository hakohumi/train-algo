fn main() {
    let binding = get_input();
    let input_string = binding.trim();

    let split_input = parse_input_strings(input_string);

    let value_list: Vec<i32> = split_input
        .iter()
        .map(|string| string.parse().unwrap())
        .collect();

    let max = calc(&value_list);

    let num = max;
    println!("{}", num);
}

fn calc(value_list: &Vec<i32>) -> &i32 {
    let ones_place = value_list
        .iter()
        .reduce(|acc, value| acc.compare_ones_place_min(value))
        .unwrap();
    ones_place
}

trait CompareOnesPlaceMin {
    fn compare_ones_place_min<'a>(&'a self, value: &'a i32) -> &'a i32;
}

impl CompareOnesPlaceMin for i32 {
    fn compare_ones_place_min<'a>(&'a self, value: &'a i32) -> &'a i32 {
        if *self % 10 < value % 10 {
            self
        } else {
            value
        }
    }
}

fn parse_input_strings(string: &str) -> Vec<&str> {
    string.split(" ").collect()
}

fn get_input() -> String {
    let mut buf: String = "".to_string();
    std::io::stdin().read_line(&mut buf).ok();
    buf
}
