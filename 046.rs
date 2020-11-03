
#[derive(Debug)]
struct Foo {
    x: i32,
}

fn do_something(f: Foo) {
    println!("{}", f.x);
}

fn main() {
    let foo  = Foo { x: 42 };
    do_something(foo);
    // println!("{:?}", foo);
}