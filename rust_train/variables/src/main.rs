use std::io;
fn inc(x: i32) -> i32 {
    x + 1
}
fn dec(x: i32) -> i32 {
    x - 1
}
fn main() {
    let x = 5;
    const MAX_POINT: u32 = 100000;
    let mut op = String::new();
    println!("The value of x is: {}", x); // x の値は{} です
    println!("The value of x is: {}", MAX_POINT);
    loop {
        io::stdin().read_line(&mut op).expect("error");
        if op == "+" {
            let x = inc(x);
            println!("value of x is: {}", x);
        } else if op == "-" {
            let x = dec(x);
            println!("value of x is: {}", x);
        } else {
            break;
        }
        //println!("value of x is: {}", x);
        //TODO do not work; fix;
    }
}
