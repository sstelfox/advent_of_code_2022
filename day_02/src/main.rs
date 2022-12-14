const INPUT_DATA: &'static [u8] = include_bytes!("../data/input");

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum RoundResult {
    Win,
    Tie,
    Lose,
}

impl RoundResult {
    fn inverse(&self) -> RoundResult {
        use RoundResult::*;

        match self {
            Win => Lose,
            Tie => Tie,
            Lose => Win,
        }
    }

    fn point_value(&self) -> usize {
        use RoundResult::*;

        match self {
            Win => 6,
            Tie => 3,
            Lose => 0,
        }
    }
}

impl From<&str> for RoundResult {
    fn from(value: &str) -> RoundResult {
        use RoundResult::*;

        match value {
            "X" => Lose,
            "Y" => Tie,
            "Z" => Win,
            _ => {
                panic!("invalid round result value");
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Throw {
    Rock,
    Paper,
    Scissors,
}

impl Throw {
    fn looses_to(&self) -> Throw {
        use Throw::*;

        match self {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock,
        }
    }

    fn point_value(&self) -> usize {
        use Throw::*;

        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }

    fn wins_against(&self) -> Throw {
        use Throw::*;

        match self {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper,
        }
    }
}

impl From<&str> for Throw {
    fn from(value: &str) -> Throw {
        match value {
            "A" | "X" => Throw::Rock,
            "B" | "Y" => Throw::Paper,
            "C" | "Z" => Throw::Scissors,
            _ => {
                panic!("invalid throw string: {:?}", value);
            }
        }
    }
}

fn choose_target_hand(throw: Throw, target_result: RoundResult) -> Throw {
    use RoundResult::*;

    match target_result {
        Tie => throw,
        Win => throw.looses_to(),
        Lose => throw.wins_against(),
    }
}

fn parse_both_as_throws(line: &str) -> (Throw, Throw) {
    let parts: Vec<&str> = line.split(" ").collect();
    (Throw::from(parts[0]), Throw::from(parts[1]))
}

fn parse_throw_results(line: &str) -> (Throw, RoundResult) {
    let parts: Vec<&str> = line.split(" ").collect();
    (Throw::from(parts[0]), RoundResult::from(parts[1]))
}

fn process_first_data(data: &[u8]) -> Vec<(usize, usize)> {
    let data = std::str::from_utf8(data).unwrap();

    data.lines()
        .map(|l| parse_both_as_throws(l))
        .map(|(other, me)| score_round(other, me))
        .collect()
}

fn process_second_data(data: &[u8]) -> Vec<(usize, usize)> {
    let data = std::str::from_utf8(data).unwrap();

    data.lines()
        .map(|l| parse_throw_results(l))
        .map(|(other, result)| (other, choose_target_hand(other, result)))
        .map(|(other, me)| score_round(other, me))
        .collect()
}

fn score_round(opponent: Throw, our_strategy: Throw) -> (usize, usize) {
    use RoundResult::*;

    let our_result = match (our_strategy, opponent) {
        (a, b) if a == b => Tie,
        (a, b) if a.looses_to() == b => Win,
        _ => Lose,
    };

    (
        our_strategy.point_value() + our_result.point_value(),
        opponent.point_value() + our_result.inverse().point_value(),
    )
}

fn main() {
    let results = process_first_data(INPUT_DATA);
    let our_total_score: usize = results.iter().map(|(ours, _)| ours).sum();
    println!("Our first total: {}", our_total_score);

    let results = process_second_data(INPUT_DATA);
    let our_total_score: usize = results.iter().map(|(ours, _)| ours).sum();
    println!("Our second total: {}", our_total_score);
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_DATA: &'static [u8] = include_bytes!("../data/sample");

    #[test]
    fn test_throw_throw_line_parser() {
        use Throw::*;

        let data = std::str::from_utf8(SAMPLE_DATA).unwrap();
        let throws: Vec<(Throw, Throw)> = data.lines().map(|l| parse_both_as_throws(l)).collect();

        assert_eq!(
            throws,
            vec![(Rock, Paper), (Paper, Rock), (Scissors, Scissors)]
        );
    }

    #[test]
    fn test_throw_result_line_parser() {
        use RoundResult::*;
        use Throw::*;

        let data = std::str::from_utf8(SAMPLE_DATA).unwrap();
        let throws: Vec<(Throw, RoundResult)> =
            data.lines().map(|l| parse_throw_results(l)).collect();

        assert_eq!(throws, vec![(Rock, Tie), (Paper, Lose), (Scissors, Win)]);
    }

    #[test]
    fn test_target_hand_selection() {
        use RoundResult::*;
        use Throw::*;

        assert_eq!(choose_target_hand(Rock, Tie), Rock);
        assert_eq!(choose_target_hand(Paper, Lose), Rock);
        assert_eq!(choose_target_hand(Scissors, Win), Rock);
    }

    #[test]
    fn test_sample_input_first() {
        let results = process_first_data(SAMPLE_DATA);
        let our_total_score: usize = results.iter().map(|(ours, _)| ours).sum();
        assert_eq!(our_total_score, 15);
    }

    #[test]
    fn test_sample_input_second() {
        let results = process_second_data(SAMPLE_DATA);
        let our_total_score: usize = results.iter().map(|(ours, _)| ours).sum();
        assert_eq!(our_total_score, 12);
    }
}
