use std::io;

fn main() {
    println!("Enter your energy consumption in kWh:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let usage: f32 = input.trim().parse().unwrap();

    let rate: f32;

    if usage > 200.0 {
        rate = 30.0;
    } else if usage > 100.0 {
        rate = 25.0;
    } else {
        rate = 20.0;
    }

    let total = usage * rate;

    println!("Your total electricity bill is ₦{}", total);
}
