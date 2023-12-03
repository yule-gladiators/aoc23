use std::collections::HashMap;
use std::fs;

fn main() {
    let file = "inputs/3.txt";
    let contents = fs::read_to_string(file).expect("Cannot read file:");

    let adj = vec![
        (-1, -1), (0, -1), (1, -1),
        (-1, 0), (1, 0),
        (-1, 1), (0, 1), (1, 1),
    ];

    let mut sum_part_numbers = 0;
    let mut sum_gear_ratios = 0;

    let mut grid = HashMap::new();
    let mut gears: HashMap<(i32, i32), Vec<i32>> = HashMap::new();
    let width;
    let height;

    let mut x = 0;
    let mut y = 0;
    for line in contents.lines() {
        x = 0;

        for char in line.chars() {
            grid.insert((x, y), char);

            x += 1;
        }

        y += 1;
    }
    width = x;
    height = y;

    fn is_symbol(c: char) -> bool {
        return !(c.is_ascii_digit() || c == '.');
    }

    for y in 0..height {
        let mut number = String::new();
        let mut is_part_number = false;
        let mut gear = (-1, -1);
        let mut maybe_gear = false;

        for x in 0..width {
            let char = grid[&(x, y)];
            if char.is_ascii_digit() {
                number.push(char);

                if !maybe_gear {
                    for (dx, dy) in &adj {
                        let adj = (x + dx, y + dy);
                        match grid.get(&adj) {
                            None => {}
                            Some(char) => {
                                if is_symbol(*char) {
                                    is_part_number = true;

                                    if *char == '*' {
                                        maybe_gear = true;
                                        gear = adj;
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            if !char.is_ascii_digit() || x == width - 1 {
                if !number.is_empty() {
                    let parsed = number.parse::<i32>().unwrap();
                    if is_part_number {
                        sum_part_numbers += parsed;

                        if maybe_gear {
                            gears.entry(gear)
                                .and_modify(|e| e.push(parsed))
                                .or_insert(vec![parsed]);
                        }
                    }
                    number.clear();
                    is_part_number = false;
                    maybe_gear = false;
                }
            }
        }
    }

    for gear in gears.values() {
        if gear.len() == 2 {
            sum_gear_ratios += gear[0] * gear[1];
        }
    }

    println!("part numbers: {sum_part_numbers}");
    println!("gear ratios: {sum_gear_ratios}");
}
