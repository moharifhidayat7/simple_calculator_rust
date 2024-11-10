use std::io::stdin;

fn main() {
    let mut first_input = String::new();
    stdin().read_line(&mut first_input).unwrap();
    let first_input: i32 = first_input.trim().parse().unwrap();

    let mut second_input = String::new();
    stdin().read_line(&mut second_input).unwrap();
    let second_input: i32 = second_input.trim().parse().unwrap();

    let result = first_input + second_input;

    println!("The result is {}", result);
}
