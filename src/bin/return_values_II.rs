fn main() {
    let mut current_meal = String::new();
    current_meal = add_flour(current_meal);
    // current_meal = add_sugar(current_meal);
    // add_salt();
}

fn add_flour(mut meal: String) -> String {
    meal.push_str("Add flour");
    meal
}
