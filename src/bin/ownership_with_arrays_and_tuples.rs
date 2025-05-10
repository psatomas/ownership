fn main(){
    let registrations = (true, false, true);
    let first = registrations.0;
    println!("{first}and {registrations:?}");

    let languages = (String::from("Rust"), String::from("JavaScript"));
    let first = &languages.0;
    println!("{first} and {languages:?}");
}