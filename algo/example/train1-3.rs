use algo::{get_input, calc_surplus};

fn main() {
    let num:i32 = get_input().trim().parse().unwrap();

    let result: i32 = calc_surplus(num, 5);
    println!("{}", result);
}
