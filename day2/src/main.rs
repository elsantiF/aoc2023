use std::fs;
use std::iter::Map;
use std::str::Lines;

fn process(lines: Lines) -> Map<Lines, fn(&str) -> (i32, i32, i32, i32)> {
    lines
        .map(|line| {
            let (id, game_text) = line.split_once(": ").unwrap();
            let (_, id) = id.split_once(" ").unwrap();
            let set: Vec<_> = game_text.split("; ").collect();

            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;

            for game in set {
                for bags in game.split(", ") {
                    let (amount, bag) = bags.split_once(' ').unwrap();

                    if bag == "red" {
                        red = red.max(amount.parse().unwrap());
                    }

                    if bag == "green" {
                        green = green.max(amount.parse().unwrap());
                    }

                    if bag == "blue" {
                        blue = blue.max(amount.parse().unwrap());
                    }
                }
            }
            println!("{:?}", (id, red, green, blue));
            (id.clone().parse::<i32>().unwrap(), red, green, blue)
        })
}

fn part1(file: String) {
    let lines = file.lines();

    let data: Vec<_> = process(lines)
        .filter(|&game| {
            game.1 <= 12 && game.2 <= 13 && game.3 <= 14
        })
        .collect();

    let mut sum = 0;
    for game in data {
        sum += game.0
    }

    println!("{:?}", sum)
}

fn part2(file: String) {
    let lines = file.lines();

    let data: Vec<_> = process(lines)
        .collect();

    let mut sum = 0;
    for game in data {
        sum += game.1 * game.2 * game.3
    }

    println!("{:?}", sum)
}

fn main() {
    let file = fs::read_to_string("part1.txt").unwrap();
    part2(file);
}
