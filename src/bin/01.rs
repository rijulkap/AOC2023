use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    
    let total: u32 = input.split("\n")
                                .map(|line|{
                                    TwoDigitNum::new()
                                    .solve_numeric(line)
                                    .get_combined()
                                })
                                .sum();
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    
    let map_f_letter: HashMap<char, Vec<usize>> = HashMap::from([
        ('z', vec![4]),
        ('o', vec![3]),
        ('t', vec![3,5]),
        ('f', vec![4]),
        ('s', vec![3,5]),
        ('e', vec![5]),
        ('n', vec![4]),
    ]);


    let map_word: HashMap<&str, char> = HashMap::from([
        ("zero", '0'),
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9')
    ]);
    
    let total: u32 = input.split("\n")
                                .map(|line|{
                                    TwoDigitNum::new()
                                    .solve_alphanumeric(&line, &map_f_letter, &map_word)
                                    .get_combined()
                                })
                                .sum();

    Some(total)
}



struct TwoDigitNum{
    first: Option<char>,
    last: Option<char>
}

impl TwoDigitNum {
    fn new() -> Self{
        TwoDigitNum { first: None, last: None}
    }

    fn get_combined(&self) -> u32{
        let x = format!("{}{}",self.first.unwrap(),self.last.unwrap())
            .parse()
            .unwrap();
        x
        
    }

    fn solve_numeric(&mut self, line: &str)-> &Self{
        
        for c in line.chars(){
            if c.is_numeric(){
                self.assign_numeric(&c);
            }
        }
        self
    }

    fn solve_alphanumeric(&mut self, line: &str, map_f_letter: &HashMap<char, Vec<usize>>, map_word: &HashMap<&str, char>)-> &Self{

        let input_len = line.len();
        
        for (ind,c) in line.chars().enumerate(){

            if c.is_numeric(){
                self.assign_numeric(&c)
            }
            
            else{
                match map_f_letter.get(&c){
                    Some(lengths)=> {
                        for word_length in lengths{
                            if ind + word_length <= input_len{
                                let parsed_word = &line[ind..ind+word_length];
                                match map_word.get(parsed_word){
                                    Some(derived_c) => {
                                        self.assign_numeric(&derived_c);
                                    },
                                    None => continue
                                }
                            }
                            else{
                                continue;
                            }
                        }

                    },
                    None=> continue
                };
            }
        }
        self
    }

    fn assign_numeric(&mut self, c: &char){
        if self.first.is_none(){
            self.first = Some(*c);
            self.last = Some(*c);
        }
        else {
            self.last = Some(*c)
        }
    }


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }
}
