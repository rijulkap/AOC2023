advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    
    let total: u32 = input.split("\r\n")
                                .map(|line|{
                                    get_twodigit_num(line)
                                })
                                .sum();
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn get_twodigit_num(line: &str) -> u32{
    
    let mut first: Option<char> = None;
    let mut last: Option<char> = None;


    for c in line.chars(){
        if c.is_numeric(){
            if first.is_none(){
                first = Some(c);
                last = Some(c);
            }
            else {
                last = Some(c)
            }
        }
    }

    format!("{}{}",first.unwrap(),last.unwrap()).parse().unwrap()
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
        assert_eq!(result, None);
    }
}
