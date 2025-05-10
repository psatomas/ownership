fn main() {
    let city =create_city();
    println!("{city}");
}

fn create_city() -> String {
    String::from("new york")
}