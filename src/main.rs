fn main() {
    println!(
        "{}",
        (|name: &str| -> std::string::String {
            let mut s = String::from("Hello, ");
            s.push_str(name);
            s.push_str("!");
            return s;
        })("world")
    );
}
