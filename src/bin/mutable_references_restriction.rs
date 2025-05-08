fn main() {
    let mut car = String::from("Red");
    let ref1 = &mut car;
    let ref2 = &mut car;
    println!("{ref2}");   
}