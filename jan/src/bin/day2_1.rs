use std::fs;

fn main() {
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let mut sum_possible = 0;

    let file = "inputs/2.txt";

    let contents = fs::read_to_string(file).expect("Cannot read file:");

    for line in contents.lines() {
        let mut possible = true;

        let (game, draws) = line.split_once(':').unwrap();
        let game_id = game.split_once(' ').unwrap().1.parse::<i32>().unwrap();

        for draw in draws.split(';') {
            for mut color in draw.split(',') {
                color = color.trim();
                match color.split_once(' ') {
                    None => {}
                    Some(parts) => {
                        let n: i32 = parts.0.parse().unwrap();
                        color = parts.1;

                        match color {
                            "red" => { if n > max_red { possible = false; } }
                            "green" => { if n > max_green { possible = false; } }
                            "blue" => { if n > max_blue { possible = false; } }
                            _ => { panic!("unmatched color: {color}") }
                        }
                    }
                }
                if !possible { break; }
            }
            if !possible { break; }
        }
        if possible { sum_possible += game_id; }
    }
    println!("{sum_possible}");
}
