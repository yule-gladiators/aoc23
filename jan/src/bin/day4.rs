use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    let file = "inputs/4.txt";
    let contents = fs::read_to_string(file).expect("Cannot read file:");

    let mut sum_points = 0;
    let mut sum_cards = 0;
    let mut cards = HashMap::new();

    for (mut card_number, line) in contents.lines().enumerate() {
        card_number += 1;

        let (_, numbers) = line.split_once(':').unwrap();
        let (winning, have) = numbers.split_once('|').unwrap();

        let mut winning_set = HashSet::new();
        for number in winning.trim().split_ascii_whitespace() {
            winning_set.insert(number.parse::<i32>().unwrap());
        }

        let mut have_set = HashSet::new();
        for number in have.trim().split_ascii_whitespace() {
            have_set.insert(number.parse::<i32>().unwrap());
        }

        let mut points = 0;
        let card_count = *cards.get(&card_number).unwrap_or(1_i32.borrow());
        cards.entry(card_number).or_insert(1);

        for (i, _winner) in winning_set.intersection(&have_set).enumerate() {
            points = if points > 0 { points * 2 } else { 1 };

            cards.entry(card_number + i + 1)
                .and_modify(|n| *n += card_count)
                .or_insert(card_count + 1);
        }
        sum_points += points
    }

    sum_cards = cards.iter().map(|(_, n)| n).sum();

    println!("points (part 1): {sum_points}");
    println!("total cards (part 2): {sum_cards}");
}
