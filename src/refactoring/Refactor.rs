pub fn refactor(){
    let x = 5;
    let b = 10;

    // Calculate the sum
    let sum = x + b;
    println!("Sum: {}", sum);

    // Calculate the product
    let product = x * b;
    println!("Product: {}", product);

    // Division (avoiding divide by zero)
    if b != 0
        {
            let div = x / b;
            println!("Division: {}", div);
        }
}
