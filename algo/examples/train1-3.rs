fn main() {
    let str = get_input().trim().to_string();

    let result = repeat_string(&str, 3);
    println!("{}", result);
}

fn repeat_string(str: &str, count: i32) -> String {
    (0..count).map(|_| str).collect()
}

fn get_input() -> String {
    let mut buf: String = "".to_string();
    std::io::stdin().read_line(&mut buf).ok();
    buf
}