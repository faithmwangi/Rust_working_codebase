use std::io;

fn main() {
    println!("Enter temperature (e.g., 36C or 100F):");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input = input.trim().to_uppercase();
    let (value, scale) = input.split_at(input.len() - 1);
    let value: f64 = value.parse().expect("Invalid number");

    match scale {
        "C" => {
            let f = value * 9.0 / 5.0 + 32.0;
            println!("{value}°C = {f:.2}°F");
        }
        "F" => {
            let c = (value - 32.0) * 5.0 / 9.0;
            println!("{value}°F = {c:.2}°C");
        }
        _ => println!("Unknown scale! Use C or F."),
    }
}

