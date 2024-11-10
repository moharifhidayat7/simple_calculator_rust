use std::io::stdin;

fn main() {
    println!("Please enter the math operation (e.g. 1+2, 2*3): ");
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    let (first_input, operation, second_input) = parse_input(input);

    match operation {
        '+' => add(first_input, second_input),
        '-' => subtract(first_input, second_input),
        '*' => multiply(first_input, second_input),
        '/' => divide(first_input, second_input),
        _ => println!("Invalid operation")
    }
    main();
}

fn parse_input(input: &str) -> (i32, char, i32) {
    let mut chars = input.chars();
    let first_input: i32 = chars.by_ref().take_while(|c| c.is_digit(10)).collect::<String>().parse().unwrap();
    let operation = chars.next().unwrap();
    let second_input: i32 = chars.skip_while(|c| c.is_whitespace()).collect::<String>().parse().unwrap();

    (first_input, operation, second_input)
}

fn add(first_input: i32, second_input: i32) {
    let result = first_input + second_input;
    println!("The result is {}", result);
}

fn subtract(first_input: i32, second_input: i32) {
    let result = first_input - second_input;
    println!("The result is {}", result);
}

fn multiply(first_input: i32, second_input: i32) {
    let result = first_input * second_input;
    println!("The result is {}", result);
}

fn divide(first_input: i32, second_input: i32) {
    if second_input == 0 {
        println!("Cannot divide by zero");
        return;
    }
    let result = first_input / second_input;
    println!("The result is {}", result);
}
