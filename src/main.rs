use std::env;
use std::i64;
fn main() {
    let args: Vec<String> = env::args().collect();
    let number_as_string: &String = &args[1];
    if number_as_string.as_str().starts_with("0x") {
        let hex_without_prefix = number_as_string.trim_start_matches("0x");
        let hex_as_decimal = i64::from_str_radix(hex_without_prefix, 16).unwrap();
        println!("{}", hex_as_decimal);
    } else {
        let hex_as_string: i32 = number_as_string.as_str().parse::<i32>().unwrap();
        println!("{hex_as_string:#x}");
    }
}
