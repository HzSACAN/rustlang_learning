fn main() {
    let hero_name = "Jack Sparrow".to_string();
    println!("{}", hero_name);

    let short_name = hero_name.replace("Jack Sparrow", "JS");
    println!("{}", short_name);
    let position = String::from("Quarter back");
    println!("{}", position);

    let greetings = "Greetings dear young and crazy brains".to_string();
    println!("{}", greetings);
    let short_greetings = greetings.get(0..10).unwrap();
    println!("{}", short_greetings);
}
