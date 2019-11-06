use std::io;
fn main() {
    println!("Fibonacci number");
    let golden_number = (1.0 + 5.0_f64.sqrt()) / 2.0;
    loop {
        let mut n = String::new();
        io::stdin().read_line(&mut n).expect("error");
        let n: f64 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let fibonacci = (golden_number.powf(n) - (1.0 - golden_number).powf(n)) / 5_f64.sqrt();
        println!("{}", fibonacci.round());
        break;
    }
}
