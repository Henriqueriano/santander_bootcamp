use std::collections::HashMap;
const CONST_SAMPLE: i32 = 32;
static mut static_sample: i32 = 32;

fn main() {
    println!("Basic data types in Rust:");
    let integer: i32 = 42;
    let unsigned_integer: u32 = 42;
    let float: f64 = 3.14;
    let boolean: bool = true; // Or false, only.
    let character: char = 'R';
    let string: &str = "Hello, Rust!";
    let tuple: (i32, f64, char) = (42, 3.14, 'R');
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    let vector: Vec<i32> = vec![1, 2, 3, 4, 5];
    let slice: &[i32] = &array;
    let string_object: String = String::from("Hello, Rust!");
    let mut hash_map: HashMap<String, i32> = HashMap::new();

    // Adding some values to the hash map
    hash_map.insert(String::from("one"), 1);
    hash_map.insert(String::from("two"), 2);
}

