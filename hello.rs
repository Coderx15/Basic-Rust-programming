fn main() {
    //.........
    //Compund data types
    let x: i32 = -5; // Signed Integer
    let y: u64 = 100; // Unsigned Integer
    println!("Signed Integer: {}", x);
    println!("Unsigned Integer: {}", y);
    let pi: f32 = 3.14; // Floating Point
    println!("Floating point: {}", pi);
    let is_rust_fun: bool = true; //Boolean
    println!("Is Rust fun? {}", is_rust_fun);
    let letter: char = 'R'; //character
    println!("Character: {}", letter);

    //.........
    //Arrays, Tuples, Slices, and Strings (Slice strings))
    let arr: [i32; 5] = [1, 2, 3, 4, 5]; // Array
    println!("Array: {:?}", arr);
    let fruits: [&str; 3] = ["Apple", "Banana", "Cherry"]; //Array of string slices
    println!("Fruits Array: {:?}", fruits);
    println!("First fruit: {}", fruits[0]);
    println!("Second fruit: {}", fruits[1]);
    println!("Third fruit: {}", fruits[2]);
    let human: (String, i32, bool) = ("Alice".to_string(), 30, true); //Tuple
    println!("Tuple: {:?}", human);
    let my_mix_tuple = ("Hello", 42, true, [1, 2, 3, 4, 5]); // Tuple with mixed types
    println!("Mixed Tuple: {:?}", my_mix_tuple);
    let my_mix_tuple2: (String, i32, bool, [i32; 5]) =
        ("Kratos".to_string(), 42, true, [1, 2, 3, 4, 5]); // Tuple with mixed types
    println!("Mixed Tuple 2: {:?}", my_mix_tuple2);
    let slice: &[i32] = &arr[1..4]; // Slice of the  number array
    println!("Slice: {:?}", slice);
    let animal_slice: &[&str] = &["Dog", "Cat", "Rabbit"]; //Slice of string slices
    println!("Animal Slice: {:?}", animal_slice);
    let book_slice: &[String] = &[
        "The Rust Programming Language".to_string(),
        "Programming in Rust".to_string(),
    ]; //Slice of Strings
    println!("Book Slice: {:?}", book_slice);
    let mut stone_cold: String = String::from("Hell, "); //Strings [growable,mutable, owned string type]
    println!("Stone Cold says : {}", stone_cold);
    stone_cold.push_str("Yeah!"); // Append to the string
    println!("Stone Cold says : {}", stone_cold);
    let slice: &str = &stone_cold[0..4]; // String slice
    println!("String Slice: {}", slice);

    //.........
    //an function/variables should be written in snake_case, while types and traits should be written in CamelCase.
    //snake_case: my_variable, my_function, my_struct
    //CamelCase: MyStruct, MyTrait, MyEnum
    //kebab-case: hello-world
    hello_word();
    tell_height(180);
    human_id("Alice", 30, 180.0);
    let _x: i32 = {
        let price: i32 = 5;
        let quantity: i32 = 10;
        price * quantity
    };
    println!("Total price: {}", _x);
    let sum: i32 = add(4, 6);
    println!("Sum: {}", sum);
    println!("value from the function add: {}", add(4, 6));

    //Calling the BMI function
    let weight: f64 = 70.0;
    let height: f64 = 1.82;
    let bmi: f64 = calculate_bmi(weight, height);
    println!("Your BMI is : {:.2}", bmi);
}

//Hoisting- can call functions anywhere in your code without worrying about the order of declaration.
fn hello_word() {
    println!("Hello, R!");
}

fn tell_height(height: u32) {
    println!("Height: {} cm", height);
}

fn human_id(name: &str, age: u32, height: f32) {
    println!(
        "My name is {}, I am {} years old and my height is {} cm.",
        name, age, height
    );
}

//Expression: Anything that returns a value.
//Statement: A line of code that performs an action but does not return a value.
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn calculate_bmi(weight_kg: f64, height_m: f64) -> f64 {
    weight_kg / (height_m * height_m)
}
