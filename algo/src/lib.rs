
pub fn calc_surplus(value: i32, div: i32) -> i32 {
    value % div
}

pub fn get_input() -> String {
    let mut buf: String = "".to_string();
    std::io::stdin().read_line(&mut buf).ok();
    buf
}