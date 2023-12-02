use std::fs;

pub fn solve() {
let file = "inputs/2.txt";
    let contents = fs::read_to_string(file).expect("Cannot read file:");

    let mut sum_powers = 0;

    for line in contents.lines() {
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        let draws = line.split_once(':').unwrap().1;

        for draw in draws.split(';') {
            for mut color in draw.split(',') {
                color = color.trim();
                match color.split_once(' ') {
                    None => {}
                    Some(parts) => {
                        let n: i32 = parts.0.parse().unwrap();
                        color = parts.1;

                        match color {
                            "red" => { if n > max_red { max_red = n; } }
                            "green" => { if n > max_green { max_green = n; } }
                            "blue" => { if n > max_blue { max_blue = n; } }
                            _ => { panic!("unmatched color: {color}") }
                        }
                    }
                }
            }
        }
        sum_powers += max_red * max_green * max_blue;
    }
    println!("{sum_powers}");
}
