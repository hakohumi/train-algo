
fn main() {
    let string_length_5 = get_input().trim().to_string();

    let result = get_mid_string(&string_length_5);
    println!("{}", result);
}

fn get_mid_string(string: &str) -> &str {
    let string_length = string.len();
    let string_mid = string_length / 2;
    &string[string_mid..string_mid+1]
}

fn get_input() -> String {
    let mut buf: String = "".to_string();
    std::io::stdin().read_line(&mut buf).ok();
    buf
}