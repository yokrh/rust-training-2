use std::io;
use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("read failed");
    s.trim().parse().ok().unwrap()
}
#[allow(dead_code)]
fn read_vec<T: FromStr>() -> Vec<T> {
    read::<String>().split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect()
}

fn main() {
    let x: i32 = read();
    let h = x / 3600;
    let m = (x % 3600) / 60;
    let s = x % 60;
    println!("{}:{}:{}", h, m, s);
}
