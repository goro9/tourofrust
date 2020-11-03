
#[derive(Debug)]
struct Foo {
    x: i32,
}

fn do_something(a: &Foo) -> &i32 {
    &a.x
}

fn main() {
    let mut foo  = Foo { x: 42 };
    let x = &mut foo.x;
    *x = 13;
    let y = do_something(&foo);
    println!("{}", y);
}