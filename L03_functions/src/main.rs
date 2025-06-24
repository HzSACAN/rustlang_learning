fn main() {
    greetings("HzSACAN");

    let value_1 = 10.0;
    let value_2 = 20.47;
    let total = sum(value_1, value_2);
    println!("The total is {}", total);

    let numbers = [3, 6, 1, 7, 0, 9, 4, 8, 4, 7, 3];
    println!("{:?}", numbers);

    let even_numbers = get_evens(&numbers);
    println!("{:?}", even_numbers);

    let odd_numbers = get_odds(&numbers);
    println!("{:?}", odd_numbers);
}

fn greetings(name: &str) {
    println!("Hello, {}!", name);
}
fn sum(x: f32, y: f32) -> f32 {
    x + y
}

fn get_odds(numbers: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();
    for n in numbers {
        if n % 3 == 0 {
            result.push(*n);
        }
    }
    return result;
}

fn get_evens(numbers: &[i32]) -> Vec<i32> {
    numbers.iter().filter(|&&n| n % 2 == 0).cloned().collect()
}
