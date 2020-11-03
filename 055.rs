static PI: f64 = 3.1415;

fn main() {
    static mut SECRET: &'static str = "swordfish";

    let msg: &'static str = "Hello world!";
    let p: &'static f64 = &PI;
    println!("{} {}", msg, p);

    unsafe {
        SECRET = "abracatabra";
        println!("{}", SECRET);
    }
}