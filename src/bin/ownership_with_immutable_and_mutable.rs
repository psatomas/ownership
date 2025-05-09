fn main() {
    let mut coffee = String::from("Mocha");
    let a = &mut coffee;
    println!("{a}");
    let b = a;
    println!("{b}");
}