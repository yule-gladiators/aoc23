use std::fs;

fn main() {
    let file = "inputs/1.txt";
    let contents = fs::read_to_string(file).expect("Cannot read file:");

    let mut sum_calibration = 0;

    let _digits1 = vec![
        (["1"], 1),
        (["2"], 2),
        (["3"], 3),
        (["4"], 4),
        (["5"], 5),
        (["6"], 6),
        (["7"], 7),
        (["8"], 8),
        (["9"], 9),
    ];

    let _digits2 = vec![
        (["one", "1"], 1),
        (["two", "2"], 2),
        (["three", "3"], 3),
        (["four", "4"], 4),
        (["five", "5"], 5),
        (["six", "6"], 6),
        (["seven", "7"], 7),
        (["eight", "8"], 8),
        (["nine", "9"], 9),
    ];

    let digits = _digits1;

    for line in contents.lines() {
        let mut first_digit_index = line.len();
        let mut first_digit = 0;
        let mut last_digit_index = 0;
        let mut last_digit = 0;

        for (digits, value) in &digits {
            for digit in digits {
                if let Some(i) = line.find(digit) {
                    if i <= first_digit_index {
                        first_digit_index = i;
                        first_digit = *value;
                    }

                    if let Some(i) = line.rfind(digit) {
                        if i >= last_digit_index {
                            last_digit_index = i;
                            last_digit = *value;
                        }
                    }
                }
            }
        }
        sum_calibration += first_digit * 10 + last_digit;
    }
    println!("{sum_calibration}");
}
