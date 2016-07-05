pub fn hello(name: Option<&str>) -> String {
    let name = name.unwrap_or("World");
    format!("Hello, {}!", name)
}
