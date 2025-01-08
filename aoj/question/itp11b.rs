use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("read failed");

    let x: i32 = line.trim().parse().unwrap();
    println!("{}", x*x*x);
}
