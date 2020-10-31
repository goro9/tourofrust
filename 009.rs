fn add(x: i32, y: i32) -> i32 {
    return x + y;
}

fn add2(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    println!("{}", add(42, 13));
    println!("{}", add2(42, 13));
}