use std::{collections::HashMap, u32};

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let mut total = 0;

    for (_, line) in input.split("\n").enumerate() {
        let line = String::from(line);

        let g = Game::new(&line);

        let solved = g.check_score();

        if solved == 0 {
            continue;
        } else {
            total = total + u32::pow(2, solved - 1)
        }
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<usize> {
    // let mut game_Store:HashMap<u32,&Game> = HashMap::new();
    let mut score_store: HashMap<u32, u32> = HashMap::new();

    for (_, line) in input.split("\n").enumerate() {
        let line = String::from(line);

        let g = Game::new(&line);

        score_store.insert(g.game_ident, g.check_score());
    }

    let mut values_to_search: Vec<u32> = score_store.keys().copied().collect();

    let mut ind = 0;
    loop {
        let got_val = score_store.get(&values_to_search[ind]).unwrap().to_owned();
        // println!("{} - {got_val}",values_to_search[ind]);

        for val in values_to_search[ind] + 1..values_to_search[ind] + 1 + got_val {
            values_to_search.push(val);
        }
        ind = ind + 1;
        if ind > values_to_search.len() - 1 {
            break;
        }
    }

    Some(values_to_search.len())
}

struct Game {
    game_ident: u32,
    numbers_winning: Vec<u32>,
    numbers_ours: Vec<u32>,
}

impl Game {
    fn new(line: &str) -> Self {
        let game_parts: Vec<&str> = line.split(":").collect();

        let game_ident = game_parts[0].split_whitespace().collect::<Vec<&str>>()[1]
            .parse()
            .unwrap();

        let numbers = game_parts[1]
            .split("|")
            .map(|numbers| {
                numbers[1..]
                    .split_whitespace()
                    .map(|num| num.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>()
            })
            .collect::<Vec<_>>();

        Game {
            game_ident: game_ident,
            numbers_winning: numbers[0].clone(),
            numbers_ours: numbers[1].clone(),
        }
    }

    fn check_score(&self) -> u32 {
        let mut ctr: u32 = 0;
        for num in &self.numbers_ours {
            if self.numbers_winning.contains(num) {
                ctr = ctr + 1;
            }
        }
        ctr
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
