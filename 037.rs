fn do_something_that_might_fail(i: i32) -> Result<f32, String> {
    if i == 42 {
        Ok(13.0)
    } else {
        Err(String::from("this is not the right number"))
    }
}

fn main() -> Result<(), String> {
    let result = do_something_that_might_fail(42);
    match result {
        Ok(v) => println!("found {}", v),
        Err(_e) => {
            return Err(String::from("something went wrong in main!"));
        }
    }

    Ok(())
}