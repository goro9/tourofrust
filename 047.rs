
#[derive(Debug)]
struct Foo {
    x: i32,
}

fn do_something() -> Foo {
    Foo { x: 42 }
}

fn main() {
    let foo  = do_something();
    println!("{:?}", foo);
}