#[derive(Debug, Clone)]
pub struct Card {
    id: usize,
    winners: Vec<usize>,
    numbers: Vec<usize>,
    amount: usize,
}

impl Card {
    pub fn build(line: &str) -> Card {
        let mut split = line.split(':');
        let id = split.next().unwrap().split_whitespace().nth(1).unwrap();
        let mut split = split.next().unwrap().split('|');
        let winners: Vec<usize> = split
            .next()
            .unwrap()
            .split_whitespace()
            .into_iter()
            .flat_map(|x| x.parse::<usize>())
            .collect();
        let numbers: Vec<usize> = split
            .next()
            .unwrap()
            .split_whitespace()
            .into_iter()
            .flat_map(|x| x.parse::<usize>())
            .collect();
        Card {
            id: id.parse().unwrap(),
            winners,
            numbers,
            amount: 1,
        }
    }
}

pub fn process_part1(input: &str) -> usize {
    let mut result: usize = 0;
    let lines = input.lines();
    let mut cards: Vec<Card> = Vec::new();
    for line in lines {
        cards.push(Card::build(line));
    }

    for card in cards {
        let mut num_winners = 0;
        for number in card.numbers {
            if card.winners.contains(&number) {
                num_winners += 1;
            }
        }
        if num_winners > 0 {
            let add = 2i32.pow(num_winners - 1);
            result += add as usize;
        }
    }
    result
}

pub fn process_part2(input: &str) -> usize {
    let lines = input.lines();
    let mut cards: Vec<Card> = Vec::new();
    for line in lines {
        cards.push(Card::build(line));
    }

    for i in 0..cards.len() {
        let mut num_winners = 0;
        for number in &cards[i].numbers {
            if cards[i].winners.contains(&number) {
                num_winners += 1;
            }
        }
        for j in 1..=num_winners {
            if i + j < cards.len() {
                cards[i + j].amount += 1 * cards[i].amount;
            }
        }
    }
    cards.iter().map(|card| card.amount).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn part1_works() {
        assert_eq!(process_part1(INPUT1), 13);
    }

    #[test]
    fn part2_works() {
        assert_eq!(process_part2(INPUT1), 30);
    }
}
