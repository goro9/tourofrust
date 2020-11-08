fn main() {
    let a: &'static str = r#"
        <div class="advice">
            Raw string is useful for some situations.
        </div>
        "#;
    println!("{}", a);
}