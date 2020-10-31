struct Marker;

fn main() {
    let _m = Marker;
    let _m_type = match _m {
        // {} => String::from("is unit"),
        _ => String::from("is not unit"),
    };
    println!("{}", _m_type);
}