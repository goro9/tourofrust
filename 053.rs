
#[derive(Debug)]
struct Foo {
    x: i32,
}

fn do_something<'a>(foo: &'a Foo) -> &'a i32 {
    &foo.x
}

fn main() {
    let mut foo  = Foo { x: 42 };
    let x = &mut foo.x;
    *x = 13;
    let y = do_something(&foo);
    println!("{}", y);
}