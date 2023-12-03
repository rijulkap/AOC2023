use std::str::Split;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    
    let red_max = 12;
    let green_max = 13;
    let blue_max = 14;
    
    let mut total = 0;
    

    let games = input.split("\n");
    
    for game in games{

        let game_split: Vec<&str> = game.split(" ").collect();

        let mut game_num = game_split[1];

        let game_num: &str = &game_num[..game_num.len()-1];

        let game_num_len = game_num.len();

        let game_num:u32 = game_num.parse().unwrap();

        let subgames = game[6+game_num_len..].split(";");

        if check_all_subgames(subgames, red_max, green_max, blue_max){
            total = total + game_num;
        }
        else{
            continue;
        }
    }
    Some(total)

}

fn check_all_subgames(subgames: Split<'_, &str>, red_max:u32, green_max:u32, blue_max:u32) -> bool{
    
    let mut g = Game::new();
    
    for subgame in subgames{
        
        g.parse_subgame(subgame);
    }

    g.check_max(red_max, green_max, blue_max)
}


fn calc_all_subgames_power(subgames: Split<'_, &str>) -> u32{
    
    let mut g = Game::new();
    
    for subgame in subgames{
        
        g.parse_subgame(subgame);
    }

    g.get_power()
}


struct Game{
    red:u32,
    green:u32,
    blue:u32
}

impl Game {
    fn new() -> Self{
        Game { red: 0, green: 0, blue: 0 }
    }

    fn parse_subgame(&mut self, subgame:&str) {

        let mut stored_val:u32 = 0;
        
        for (ind, vals) in subgame[1..].split(" ").enumerate(){

            if ind % 2 == 0{
                stored_val = vals.parse().unwrap();
            }
            else{
                match vals.chars().nth(0).unwrap(){
                    'r' => if stored_val > self.red{
                        self.red = stored_val;
                        stored_val = 0;
                    },
                    'g' => if stored_val > self.green{
                        self.green = stored_val;
                        stored_val = 0;
                    },
                    'b' => if stored_val > self.blue{
                        self.blue = stored_val;
                        stored_val = 0;
                    },  
                    _ => continue                  
                }
            }
        }
    }

    fn check_max(&self, rmax:u32 ,gmax:u32 ,bmax:u32) -> bool{
        if self.red <= rmax && self.blue <= bmax && self.green <=gmax{
            return true
        }
        else{
            return false
        }
    }

    fn get_power(&self)->u32{
        self.red*self.blue*self.green
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    
    let red_max = 12;
    let green_max = 13;
    let blue_max = 14;
    
    let mut total = 0;
    
    let games = input.split("\n");
    
    for game in games{

        let game_split: Vec<&str> = game.split(" ").collect();

        let mut game_num = game_split[1];

        let game_num: &str = &game_num[..game_num.len()-1];

        let game_num_len = game_num.len();

        let game_num:u32 = game_num.parse().unwrap();

        let subgames = game[6+game_num_len..].split(";");

        total = total +  calc_all_subgames_power(subgames);
    }
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some((8)));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
