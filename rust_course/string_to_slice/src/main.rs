fn main() {
    let my_string: String = String::from("hello");
    let substring: &str = &my_string[0..5];
    println!("{}", my_string);
    println!("slice: {}", substring);

    // Directional convert:
    let foo: String = String::from("My string!");
    let bar: &str = foo.as_str();

    // Or
    let another: &str = &foo;
    println!("direct convert: {}", another);
}
