pub fn process_part1(input: &str) -> u32 {
  #[derive(Debug)]
  pub struct Round {
    theirs: char,
    own: char,
  }

  impl Round {
    fn build(line: &str) -> Round {
      let line: Vec<char> = line.chars().collect();
      let theirs = line[0];
      let own = line[2];

      Round { theirs, own }
    }
  }

  let mut rounds: Vec<Round> = Vec::new();

  for line in input.lines() {
    rounds.push(Round::build(line));
  }

  let mut score: u32 = 0;
  for round in &rounds {
    score += match round.own {
      'X' => 1,
      'Y' => 2,
      'Z' => 3,
      _other => panic!(),
    };
  }

  for round in &rounds {
    score += match round {
      Round {
        theirs: 'A',
        own: 'X',
      } => 3,
      Round {
        theirs: 'A',
        own: 'Y',
      } => 6,
      Round {
        theirs: 'A',
        own: 'Z',
      } => 0,
      Round {
        theirs: 'B',
        own: 'X',
      } => 0,
      Round {
        theirs: 'B',
        own: 'Y',
      } => 3,
      Round {
        theirs: 'B',
        own: 'Z',
      } => 6,
      Round {
        theirs: 'C',
        own: 'X',
      } => 6,
      Round {
        theirs: 'C',
        own: 'Y',
      } => 0,
      Round {
        theirs: 'C',
        own: 'Z',
      } => 3,
      _other => panic!(),
    }
  }
  score
}

pub fn process_part2(input: &str) -> u32 {
  #[derive(Debug)]
  pub struct Round {
    theirs: char,
    own: char,
  }

  impl Round {
    fn build(line: &str) -> Round {
      let line: Vec<char> = line.chars().collect();
      let theirs = line[0];
      let own = line[2];

      Round { theirs, own }
    }
  }

  let mut rounds: Vec<Round> = Vec::new();

  for line in input.lines() {
    rounds.push(Round::build(line));
  }

  let mut score: u32 = 0;

  for round in &rounds {
    score += match round {
      Round {
        theirs: 'A',
        own: 'X',
      } => 3,
      Round {
        theirs: 'A',
        own: 'Y',
      } => 4,
      Round {
        theirs: 'A',
        own: 'Z',
      } => 8,
      Round {
        theirs: 'B',
        own: 'X',
      } => 1,
      Round {
        theirs: 'B',
        own: 'Y',
      } => 5,
      Round {
        theirs: 'B',
        own: 'Z',
      } => 9,
      Round {
        theirs: 'C',
        own: 'X',
      } => 2,
      Round {
        theirs: 'C',
        own: 'Y',
      } => 6,
      Round {
        theirs: 'C',
        own: 'Z',
      } => 7,
      _other => panic!(),
    }
  }
  score
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn part1_test() {
    let input = "\
A Y
B X
C Z";
    assert_eq!(process_part1(input), 15)
  }

  #[test]
  fn part2_test() {
    let input = "\
A Y
B X
C Z";
    assert_eq!(process_part2(input), 12)
  }
}
