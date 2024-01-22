#[derive(Debug, Clone)]
pub struct Hand {
    pub cards: Vec<char>,
    pub hand_type: HandTypes,
    pub bid: usize,
}

impl Hand {
    pub fn new(line: &str) -> Hand {
        let cards: Vec<char> = line.split_whitespace().next().unwrap().chars().collect();
        let bid: usize = line
            .split_whitespace()
            .nth(1)
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let hand_type: HandTypes = HandTypes::Five;
        dbg!(&cards, &bid);
        Hand {
            cards,
            hand_type,
            bid,
        }
    }
}

#[derive(Debug, Clone)]
pub enum HandTypes {
    Five,
    Four,
    FullHouse,
    Three,
    TwoPair,
    OnePair,
    HighCard,
}

pub fn process_part1(input: &str) -> usize {
    let result: usize = 0;
    let _hands: Vec<Hand> = input.lines().map(|line| Hand::new(line)).collect();
    result
}

pub fn process_part2(_input: &str) -> usize {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn part1_works() {
        assert_eq!(process_part1(INPUT1), 6440);
    }

    #[test]
    fn part2_works() {
        todo!();
    }
}
