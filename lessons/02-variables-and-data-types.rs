fn main() {
    // 1. Declare variables
    let name = "Aurel";
    let language = "Rust";

    // 2. Declare a mutable variable
    let mut projects = 1;

    // 3. Increment the variable by 1
    projects += 1;

    // 4. Declare variables with explicit data types
    let age: i32 = 18;
    let height: f64 = 1.81;
    let is_learning: bool = true;
    let initial: char = 'A';

    // 5. Print the variable values
    println!("=== Developer Profile ===");
    println!("Name: {name}");
    println!("Language: {language}");
    println!("Projects: {projects}");
    println!("Age: {age}");
    println!("Height: {height}");
    println!("Learning Rust: {is_learning}");
    println!("Initial: {initial}");
}
