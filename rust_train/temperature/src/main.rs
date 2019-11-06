use std::io;
fn main() {
    println!("input celsius temperature");
    loop {
        let mut temp = String::new();
        io::stdin().read_line(&mut temp).expect("error");
        let temp: f64 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("celsius temperature: {}", temp);
        println!("fahrenheit temperature: {}", (temp * 9.0 / 5.0) + 32.0);
        println!("celsius temperature: {}", temp + 273.15);
        break;
    }
}
