use proconio::input;

fn main() {
    input! {
        N: i64,
    }
    let mut res: i64 = 1; //mutable
    for i in 1..N+1 {
        res *= i;
    }
    println!("{}", res);
}
