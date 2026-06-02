use std::io;

fn main() {
    // The user input stay in this variable.
    print!("Insert value:\n");
    let mut input: String = String::new();

    // Getting user input!
    io::stdin()
    .read_line(&mut input)
    .expect("Error while reading user input!");
    
    // Convert
    let input: i32 = input.trim().parse().expect("Please, insert a valid number!");
    println!("User input is {}!", input);
}
