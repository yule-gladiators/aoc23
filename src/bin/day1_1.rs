use std::fs;

fn main() {
    let file = "inputs/1.txt";
    let contents = fs::read_to_string(file).expect("Cannot read file:");

    let mut sum_calibration = 0;

    for line in contents.lines() {
        let mut first_digit = ' ';
        let mut last_digit = ' ';

        for char in line.chars() {
            if char.is_ascii_digit() {
                first_digit = char;
                break;
            }
        }

        for char in line.chars().rev() {
            if char.is_ascii_digit() {
                last_digit = char;
                break;
            }
        }

        // first_digit = line.chars().nth(line.find(|c: char| c.is_ascii_digit()).unwrap()).unwrap();
        // last_digit = line.chars().nth(line.rfind(|c: char| c.is_ascii_digit()).unwrap()).unwrap();

        let calibration: String = first_digit.to_string() + &last_digit.to_string();
        sum_calibration += calibration.parse::<i32>().unwrap();
    }
    println!("{sum_calibration}");
}
