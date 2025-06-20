fn main() {
    let name = String::from("Hazret Saçan");
    println!("Hello, {name}");

    let mut player_score = 51;
    player_score += 1;
    println!("Oyuncu skoru: {player_score} ");

    let _delta_time = 1.25;
    let _delta_time: f32 = 1.25;
    println!("Current delta time: {_delta_time}");

    let color_in_hex = 0xFF0033;
    println!("Background color is {color_in_hex:x} or {color_in_hex}");

    let dir_permission = 0o755;
    println!("Directory permission is {dir_permission:o} / {dir_permission}");

    let gate_flag: u8 = 0b1010_0100;
    println!("Gate flag is {gate_flag:b} / {gate_flag}");

    let config = (640, 480, String::from("Main Title"), false);
    println!("The config is {config:?}");

    let width = config.0;
    let _height = config.1;
    let (w, h) = (width, config.1);
    println!("The screen resolution is {w}:{h}");

    let mut scores: [u8; 5] = [56, 10, 90, 100, 48];
    println!("The scores are {scores:?}");
    println!(
        "The first score is {}. Length is {}",
        scores[0],
        scores.len()
    );
    scores[1] += 50;
    println!("The scores are {scores:?}");

    println!("Background color is {BACKGROUND_COLOR:?}");
}

const BACKGROUND_COLOR: (u8, u8, u8) = (0xff, 0xff, 0xff);
