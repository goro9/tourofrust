fn main() {
    let mut x = 0;
    loop {
        x += 1;
        // x++;
        if x == 42 {
            break;
        }
    }
    println!("{}", x);
}