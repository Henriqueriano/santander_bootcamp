use std::io;

fn main() {
    /* 
        The user input stay in this variable.
        Note: for multiple line prints, the dev
        can use the println!(r#"
        
        "#); macro.

        Note the r# moddifier.
    */
    print!("Insert value:\n");
    let mut input: String = String::new();

    // Getting user input!
    io::stdin()
    .read_line(&mut input)
    .expect("Error while reading user input!");
    
    // Convert
    //Looks, the shadowing concept! xD
    let input: i32 = input.trim().parse().expect("Please, insert a valid number!");
    println!("User input is {}!", input);
}
