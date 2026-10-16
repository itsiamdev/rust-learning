
fn main() {
    // 1. Basic if statement
    let age = 20;

    if age >= 18 {
        println!("You are an adult.");
    }

    // 2. If-else statement
    let number = 7;

    if number % 2 == 0 {
        println!("The number is even.");
    } else {
        println!("The number is odd.");
    }

    // 3. If-else if-else statement
    let score = 85;

    if score >= 90 {
        println!("Grade: A");
    } else if score >= 80 {
        println!("Grade: B");
    } else if score >= 70 {
        println!("Grade: C");
    } else {
        println!("Grade: D");
    }

    // 4. Logical operators in conditions
    let user_age = 20;
    let has_ticket = true;

    if user_age >= 18 && has_ticket {
        println!("Access granted.");
    } else {
        println!("Access denied.");
    }

    // 5. Multiple conditions with OR
    let day = 6;

    if day == 6 || day == 7 {
        println!("It is the weekend.");
    } else {
        println!("It is a weekday.");
    }

    // 6. If as an expression
    let temperature = 25;

    let weather = if temperature > 20 {
        "Warm"
    } else {
        "Cold"
    };

    println!("Weather: {}", weather);

    // 7. Nested if statements
    let age = 20;
    let has_id = true;

    if age >= 18 {
        if has_id {
            println!("You can enter.");
        } else {
            println!("You need an ID.");
        }
    } else {
        println!("You are too young.");
    }
}
