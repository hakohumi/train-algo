
fn main() {
    let time:i32 = get_input().trim().parse().unwrap();

    let result = calc_diff_time(time);
    println!("{}", result);
}

fn calc_diff_time(now: i32) -> i32 {
    const DAY_TIME: i32 = 24;

    DAY_TIME - now
}

fn get_input() -> String {
    let mut buf: String = "".to_string();
    std::io::stdin().read_line(&mut buf).ok();
    buf
}