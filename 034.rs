#[derive(Debug)]
enum Item {
    Inventory(String),
    None,
}

#[derive(Debug)]
struct BagOfHolding {
    item: Item,
}

fn main() {
    let bag = BagOfHolding {
        item: Item::Inventory(String::from("chocolate"))
    };
    let empty_bag = BagOfHolding {
        item: Item::None,
    };
    println!("{:?}", bag);
    println!("{:?}", empty_bag);
}