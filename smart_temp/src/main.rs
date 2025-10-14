use std::io;

fn main() {
    println!("Type C to convert Celsius → Fahrenheit");
    println!("Type F to convert Fahrenheit → Celsius");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    let choice = choice.trim();

    println!("Enter the temperature:");
    let mut temp = String::new();
    io::stdin().read_line(&mut temp).unwrap();
    let temp: f64 = temp.trim().parse().unwrap();

    if choice == "C" || choice == "c" {
        let result = (temp * 9.0 / 5.0) + 32.0;
        println!("Temperature: {:.1}°C", temp);
        println!("Converted: {:.1}°F", result);
    } else if choice == "F" || choice == "f" {
        let result = (temp - 32.0) * 5.0 / 9.0;
        println!("Temperature: {:.1}°F", temp);
        println!("Converted: {:.1}°C", result);
    } else {
        println!("Invalid input!");
    }
}
