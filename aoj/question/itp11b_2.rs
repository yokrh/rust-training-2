use std::io;
use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("read failed");
    s.trim().parse().ok().unwrap()
}
fn read_vec<T: FromStr>() -> Vec<T> {
    read::<String>().split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect()
}

fn main() {
    let v: Vec<i32> = read_vec();
    let x = v[0];
    println!("{}", x.pow(3));
}