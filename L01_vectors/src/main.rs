fn main() {
    let mut scores: Vec<u16> = vec![30, 40, 20, 10, 80];
    scores.push(90);
    scores.push(100);
    println!("{scores:?}");

    for s in scores.iter() {
        println!("{}", s);
    }

    let last_score = scores.pop().unwrap();

    let a = scores.len();
    println!(
        "{:?} is last point of vectors but does not exist. Current last point is {}",
        last_score,
        scores[a - 1]
    );

    let mut colors = Vec::new();
    colors.push(String::from("red"));
    colors.push(String::from("green"));
    colors.push(String::from("blue"));
}
