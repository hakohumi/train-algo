fn main() {
    let split_input = get_input(3);
    let result = reverse_concat(split_input);
    println!("{}", result);
}

fn reverse_concat(list: Vec<String>) -> String {
    let mut list = list;
    list.reverse();
    list.join("")
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
