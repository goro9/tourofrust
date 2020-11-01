fn main() {
    let mut i32_vec = Vec::<i32>::new();
    i32_vec.push(1);
    i32_vec.push(2);
    i32_vec.push(3);

    println!("{:?}", i32_vec);

    let mut float_vec = Vec::new();
    float_vec.push(1.3);
    float_vec.push(2.3);
    float_vec.push(3.4);

    println!("{:?}", float_vec);

    let string_vec = vec![String::from("Hello"), String::from("World")];

    for word in string_vec.iter() {
        println!("{}", word);
    }
}