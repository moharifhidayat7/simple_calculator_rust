use std::io::stdin;

fn main() {
    println!("Please enter the first number: ");
    let mut first_input = String::new();
    stdin().read_line(&mut first_input).unwrap();
    let first_input: i32 = first_input.trim().parse().unwrap();

    println!("Please enter the second number: ");
    let mut second_input = String::new();
    stdin().read_line(&mut second_input).unwrap();
    let second_input: i32 = second_input.trim().parse().unwrap();

    println!("Choose the operation: ");
    println!("1. Add");
    println!("2. Subtract");
    println!("3. Multiply");
    println!("4. Divide");

    let mut operation = String::new();
    stdin().read_line(&mut operation).unwrap();
    let operation: i32 = operation.trim().parse().unwrap();
    
    match operation {
        1 => add(first_input, second_input),
        2 => subtract(first_input, second_input),
        3 => multiply(first_input, second_input),
        4 => divide(first_input, second_input),
        _ => println!("Invalid operation")
    }
    main();
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
