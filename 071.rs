fn main() -> Result<(), std::num::ParseIntError> {
    let a = 42;
    let a_string = a.to_string();
    let b = a_string.parse::<i32>()?;
    println!("{} {} {}", a, b, a_string);
    println!("{} {} {}", type_of(a), type_of(b), type_of(a_string));
    Ok(())
}

fn type_of<T>(_: T) -> &'static str {
    std::any::type_name::<T>()
}