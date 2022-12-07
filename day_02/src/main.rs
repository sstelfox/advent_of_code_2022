const INPUT_DATA: &'static [u8] = include_bytes!("../data/input");

#[derive(Debug, Eq, PartialEq)]
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
            _ => { panic!("invalid round result value"); }
        }
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

fn score_round(opponent: &Throw, our_strategy: &Throw) -> (usize, usize) {
    use RoundResult::*;
    use Throw::*;

    let our_result = match (our_strategy, opponent) {
        (a, b) if a == b => Tie,
        (Rock, Scissors) => Win,
        (Paper, Rock) => Win,
        (Scissors, Paper) => Win,
        _ => Lose,
    };

    (
        our_strategy.point_value() + our_result.point_value(),
        opponent.point_value() + our_result.inverse().point_value(),
    )
}

#[derive(Debug, Eq, PartialEq)]
enum Throw {
    Rock,
    Paper,
    Scissors,
}

impl Throw {
    fn point_value(&self) -> usize {
        use Throw::*;

        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
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

fn process_first_data(data: &[u8]) -> Vec<(usize, usize)> {
    let data = std::str::from_utf8(data).unwrap();

    data.lines()
        .map(|l| parse_both_as_throws(l))
        .map(|(other, me)| score_round(&other, &me))
        .collect()
}

fn main() {
    let results = process_first_data(INPUT_DATA);
    let our_total_score: usize = results.iter().map(|(ours, _)| ours).sum();
    println!("Our total: {}", our_total_score);
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
        use Throw::*;
        use RoundResult::*;

        let data = std::str::from_utf8(SAMPLE_DATA).unwrap();
        let throws: Vec<(Throw, RoundResult)> = data.lines().map(|l| parse_throw_results(l)).collect();

        assert_eq!(
            throws,
            vec![(Rock, Tie), (Paper, Lose), (Scissors, Win)]
        );
    }

    #[test]
    fn test_sample_input_first() {
        let results = process_first_data(SAMPLE_DATA);
        let our_total_score: usize = results.iter().map(|(ours, _)| ours).sum();
        assert_eq!(our_total_score, 15);
    }
}
