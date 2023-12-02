use std::collections::HashMap;
use std::fs;

pub fn solve() {
    let file = "inputs/1.txt";
    let contents = fs::read_to_string(file).expect("Cannot read file:");

    let mut sum_calibration = 0;

    let mut digits = HashMap::with_capacity(9);
    digits.insert("one", '1');
    digits.insert("two", '2');
    digits.insert("three", '3');
    digits.insert("four", '4');
    digits.insert("five", '5');
    digits.insert("six", '6');
    digits.insert("seven", '7');
    digits.insert("eight", '8');
    digits.insert("nine", '9');

    for line in contents.lines() {
        let mut first_digit_index = line.len();
        let mut last_digit_index = 0;
        let mut last_digit = ' ';
        let mut first_digit: char = ' ';

        for digit in &digits {
            if let Some(i) = line.find(digit.0) {
                if i <= first_digit_index {
                    first_digit_index = i;
                    first_digit = *digit.1;
                }
            }
        }

        if let Some(i) = line.find(|c: char| c.is_ascii_digit()) {
            if i <= first_digit_index {
                first_digit = line.chars().nth(i).unwrap();
            }
        }

        for digit in &digits {
            if let Some(i) = line.rfind(digit.0) {
                if i >= last_digit_index {
                    last_digit_index = i;
                    last_digit = *digit.1;
                }
            }
        }

        if let Some(i) = line.rfind(|c: char| c.is_ascii_digit()) {
            if i >= last_digit_index {
                last_digit = line.chars().nth(i).unwrap();
            }
        }

        let calibration: String = first_digit.to_string() + &last_digit.to_string();
        sum_calibration += calibration.parse::<i32>().unwrap();
    }
    println!("{sum_calibration}");
}
