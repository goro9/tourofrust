struct BagOfHolding<T> {
    item: Option<T>,
}

fn main() {
    let i32_bag = BagOfHolding::<i32> { item: None };
    if i32_bag.item.is_none() {
        println!("there's nothing in the bag!")
    } else {
        println!("there's something in the bag!")
    }

    match i32_bag.item {
        Some(v) => println!("found {} in bag!", v),
        None => println!("found nothing"),
    }

    let i32_bag = BagOfHolding::<i32> { item: Some(42) };
    if i32_bag.item.is_some() {
        println!("there's something in the bag!")
    } else {
        println!("there's nothing in the bag!")
    }
    
    match i32_bag.item {
        Some(v) => println!("found {} in bag!", v),
        None => println!("found nothing"),
    }
}