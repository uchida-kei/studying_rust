use std::io;
fn main() {
    println!("Fibonacci number");
    const FIB_ZERO: u64 = 0;
    const FIB_ONE: u64 = 1;
    let mut fib_vec = vec![FIB_ZERO, FIB_ONE];
    loop {
        let mut n = String::new();
        io::stdin().read_line(&mut n).expect("error");
        let n: usize = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        for i in 0..n + 1 {
            if i != 0 && i != 1 {
                fib_vec.push(fib_vec[fib_vec.len() - 1] + fib_vec[fib_vec.len() - 2]);
            }
        }
        println!("fib_num: {}", fib_vec[n]);
        break;
    }
}
