use core::num;
use std::{collections::HashMap, u32};

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    
    let mut total = 0;

    for (line_num, line) in input.split("\n").enumerate() {
        let line = String::from(line);

        let g = Game::new(&line);
        
        let solved =  g.check_score();

        if solved == 0{
            continue;
        }
        else{
            total = total + u32::pow(2, solved-1)
        }
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut total = 0;

    // let mut game_Store:HashMap<u32,&Game> = HashMap::new();
    let mut score_Store:HashMap<u32,u32> = HashMap::new();

    for (line_num, line) in input.split("\n").enumerate() {
        let line = String::from(line);

        let g = Game::new(&line);

        score_Store.insert(g.game_ident, g.check_score());
    }
    println!("{:?}",score_Store);

    for upper_key in score_Store.keys(){
        // println!("{}  {}",upper_key, calculate_total(upper_key, &score_Store));
        // break
        println!("{upper_key}");
        total = total + calculate_total(upper_key, &score_Store);
    }

    Some(total)     
}

fn calculate_total(key:&u32, map:&HashMap<u32,u32>) -> u32{
        let mut totals = map.get(key).unwrap().to_owned();
        
        if totals == 0{
            return 0 
        }
        else{
            for val in key+1..key+1+totals{
                totals = totals + calculate_total(&val, map);
                println!("Calling key {} -- inner {} -- totals {}",key,val,totals);
            }
            return totals
        }

}

struct Game {
    game_ident: u32,
    numbers_winning: Vec<u32>,
    numbers_ours: Vec<u32>
}

impl Game {
    fn new(line: &str) -> Self {
        let mut game_parts: Vec<&str> = line.split(":").collect();

        let game_ident = game_parts[0].split_whitespace().collect::<Vec<&str>>()[1].parse().unwrap();

        let numbers = game_parts[1].split("|").map(|numbers| {
            numbers[1..]
                .split_whitespace()
                .map(|num| num.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        }).collect::<Vec<_>>();

        Game {
            game_ident: game_ident,
            numbers_winning : numbers[0].clone(),
            numbers_ours : numbers[1].clone()
        }
    }

    fn check_score(&self) -> u32{
        
        let mut ctr:u32 = 0;
        for num in &self.numbers_ours{
            if self.numbers_winning.contains(num){
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
