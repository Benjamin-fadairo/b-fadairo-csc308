use std::io;

fn main() {
    println!("Enter the total bill amount:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let amount: f64 = input.trim().parse().unwrap();

    let final_amount = if amount > 10000.0 {
        amount - (amount * 0.15)
    } else if amount > 5000.0 {
        amount - (amount * 0.10)
    } else {
        amount
    };

    println!("Final bill after discount: ₦{ }", final_amount);
}
