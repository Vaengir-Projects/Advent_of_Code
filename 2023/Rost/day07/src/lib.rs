use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hand {
    pub cards: Vec<usize>,
    pub hand_type: HandType,
    pub bid: usize,
}

impl Hand {
    pub fn new(line: &str) -> Hand {
        let cards: Vec<char> = line.split_whitespace().next().unwrap().chars().collect();
        let card_values: Vec<usize> = cards
            .iter()
            .map(|&c| match c {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 11,
                'T' => 10,
                '9' => 9,
                '8' => 8,
                '7' => 7,
                '6' => 6,
                '5' => 5,
                '4' => 4,
                '3' => 3,
                '2' => 2,
                _ => panic!("Input not a valid card"),
            })
            .collect();
        let hand_type: HandType;
        let bid: usize = line
            .split_whitespace()
            .nth(1)
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let mut char_counts = HashMap::new();
        for &c in &cards {
            let count = char_counts.entry(c).or_insert(0);
            *count += 1;
        }
        if char_counts.len() == 1 {
            hand_type = HandType::Five;
        } else if char_counts.len() == 2 && char_counts.values().any(|&c| c == 4) {
            hand_type = HandType::Four;
        } else if char_counts.len() == 2 && char_counts.values().any(|&c| c == 3) {
            hand_type = HandType::FullHouse;
        } else if char_counts.len() == 3 && char_counts.values().any(|&c| c == 3) {
            hand_type = HandType::Three;
        } else if char_counts.len() == 3 && char_counts.values().any(|&c| c == 2) {
            hand_type = HandType::TwoPair;
        } else if char_counts.len() == 4 && char_counts.values().any(|&c| c == 2) {
            hand_type = HandType::OnePair;
        } else {
            hand_type = HandType::HighCard;
        }
        Hand {
            cards: card_values,
            hand_type,
            bid,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.hand_type
            .cmp(&other.hand_type)
            .then(self.cards.cmp(&other.cards))
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HandType {
    Five,
    Four,
    FullHouse,
    Three,
    TwoPair,
    OnePair,
    HighCard,
}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (HandType::Five, HandType::Four)
            | (HandType::Five, HandType::FullHouse)
            | (HandType::Five, HandType::Three)
            | (HandType::Five, HandType::TwoPair)
            | (HandType::Five, HandType::OnePair)
            | (HandType::Five, HandType::HighCard) => std::cmp::Ordering::Greater,
            (HandType::Five, HandType::Five) => std::cmp::Ordering::Equal,
            (HandType::Four, HandType::FullHouse)
            | (HandType::Four, HandType::Three)
            | (HandType::Four, HandType::TwoPair)
            | (HandType::Four, HandType::OnePair)
            | (HandType::Four, HandType::HighCard) => std::cmp::Ordering::Greater,
            (HandType::Four, HandType::Four) => std::cmp::Ordering::Equal,
            (HandType::FullHouse, HandType::Three)
            | (HandType::FullHouse, HandType::TwoPair)
            | (HandType::FullHouse, HandType::OnePair)
            | (HandType::FullHouse, HandType::HighCard) => std::cmp::Ordering::Greater,
            (HandType::FullHouse, HandType::FullHouse) => std::cmp::Ordering::Equal,
            (HandType::Three, HandType::TwoPair)
            | (HandType::Three, HandType::OnePair)
            | (HandType::Three, HandType::HighCard) => std::cmp::Ordering::Greater,
            (HandType::Three, HandType::Three) => std::cmp::Ordering::Equal,
            (HandType::TwoPair, HandType::OnePair) | (HandType::TwoPair, HandType::HighCard) => {
                std::cmp::Ordering::Greater
            }
            (HandType::TwoPair, HandType::TwoPair) => std::cmp::Ordering::Equal,
            (HandType::OnePair, HandType::HighCard) => std::cmp::Ordering::Greater,
            (HandType::OnePair, HandType::OnePair) => std::cmp::Ordering::Equal,
            (HandType::HighCard, HandType::HighCard) => std::cmp::Ordering::Equal,
            (HandType::HighCard, _) => std::cmp::Ordering::Less,
            _ => std::cmp::Ordering::Less,
        }
    }
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn process_part1(input: &str) -> usize {
    let mut result: usize = 0;
    let mut hands: Vec<Hand> = input.lines().map(Hand::new).collect();
    hands.sort();
    for (i, hand) in hands.iter().enumerate() {
        result += (i + 1) * hand.bid;
    }
    result
}

impl Hand {
    pub fn new2(line: &str) -> Hand {
        let cards: Vec<char> = line.split_whitespace().next().unwrap().chars().collect();
        let card_values: Vec<usize> = cards
            .iter()
            .map(|&c| match c {
                'A' => 13,
                'K' => 12,
                'Q' => 11,
                'T' => 10,
                '9' => 9,
                '8' => 8,
                '7' => 7,
                '6' => 6,
                '5' => 5,
                '4' => 4,
                '3' => 3,
                '2' => 2,
                'J' => 1,
                _ => panic!("Input not a valid card"),
            })
            .collect();
        let hand_type: HandType;
        let bid: usize = line
            .split_whitespace()
            .nth(1)
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let mut char_counts = HashMap::new();
        for &c in &cards {
            let count = char_counts.entry(c).or_insert(0);
            *count += 1;
        }
        let jokers = char_counts.remove(&'J').unwrap_or(0);
        if char_counts.values().any(|&c| c + jokers == 5) || jokers == 5 {
            hand_type = HandType::Five;
        } else if char_counts.values().any(|&c| c + jokers == 4) {
            hand_type = HandType::Four;
        } else if (char_counts.values().filter(|&c| c == &2).count() == 2 && jokers == 1)
            || (char_counts.values().any(|&c| c == 3) && char_counts.values().any(|&c| c == 2))
        {
            hand_type = HandType::FullHouse;
        } else if char_counts.values().any(|&c| c + jokers == 3)
            && char_counts.values().filter(|&c| c == &2).count() <= 1
        {
            hand_type = HandType::Three;
        } else if char_counts.values().filter(|&c| c == &2).count() == 2 && jokers == 0 {
            hand_type = HandType::TwoPair;
        } else if char_counts.len() == 4 && char_counts.values().any(|&c| c == 2)
            || char_counts.len() == 4 && jokers == 1
        {
            hand_type = HandType::OnePair;
        } else {
            hand_type = HandType::HighCard;
        }
        Hand {
            cards: card_values,
            hand_type,
            bid,
        }
    }
}

pub fn process_part2(input: &str) -> usize {
    let mut result: usize = 0;
    let mut hands: Vec<Hand> = input.lines().map(Hand::new2).collect();
    hands.sort();
    for (i, hand) in hands.iter().enumerate() {
        result += (i + 1) * hand.bid;
    }
    result
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
        assert_eq!(process_part2(INPUT1), 5905);
    }
}
