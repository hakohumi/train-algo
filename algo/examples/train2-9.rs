fn main() {
    let split_input = get_input(2);
    let string:String = split_input[0].clone();
    let index = split_input[1].parse().unwrap();
    let result: &str = &string.as_str()[index-1..index];
    println!("{}", result);
}

fn get_input(num: i32) -> Vec<String> {
    let mut input_list: Vec<String> = vec![];
    for _ in 0..num {
        let mut _buf: String = "".to_string();
        std::io::stdin().read_line(&mut _buf).ok();
        input_list.push(_buf.trim().to_string());
    }
    input_list
}
