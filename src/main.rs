trait HelloStr {
    fn hello_str(&self) -> &str;
}

impl HelloStr for &str {
    fn hello_str(&self) -> &str {
        return self;
    }
}

impl HelloStr for String {
    fn hello_str(&self) -> &str {
        return &self;
    }
}

fn hello(name: impl HelloStr) -> std::string::String {
    let mut s = String::from("Hello, ");
    s.push_str(name.hello_str());
    s.push_str("!");
    return s;
}

fn main() {
    println!("{}", hello("world"));
    println!("{}", hello(String::from("world")));
}
