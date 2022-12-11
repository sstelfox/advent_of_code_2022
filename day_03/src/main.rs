const INPUT_DATA: &'static [u8] = include_bytes!("../data/input");

fn process_day_one(data: &[u8]) -> bool {
    let data = std::str::from_utf8(data).unwrap();

    //data.lines()
    //    .map(|l| parse_both_as_throws(l))
    //    .map(|(other, me)| score_round(other, me))
    //    .collect()

    false
}

fn main() {
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_DATA: &'static [u8] = include_bytes!("../data/sample");

    #[test]
    fn test_sample_data_result() {
        assert!(process_day_one(SAMPLE_DATA), true);
    }
}
