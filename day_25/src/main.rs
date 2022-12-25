const INPUT_DATA: &'static [u8] = include_bytes!("../data/input");

struct Snafu {
    base_10_value: isize,
}

impl Snafu {
    fn base10(&self) -> isize {
        self.base_10_value
    }

    fn snafu(&self) -> String {
        unimplemented!()
    }
}

impl std::ops::Add<&Snafu> for &Snafu {
    type Output = Snafu;

    fn add(self, other: Self) -> Self::Output {
        Snafu {
            base_10_value: self.base_10_value + other.base_10_value,
        }
    }
}

fn parse_input(data: &[u8]) -> Vec<Snafu> {
    let data = std::str::from_utf8(data).unwrap();
    data.lines().map(|l| parse_line(l)).collect()
}

fn parse_line(line: &str) -> Snafu {
    unimplemented!()
}

fn main() {
    let snafus = parse_input(INPUT_DATA);
    let result: Snafu = snafus.iter().sum();

    println!("The sum of the input is {} in base10, or {} in SNAFU", result.base10(), result.snafu());
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &'static [u8] = include_bytes!("../data/sample");

    #[test]
    fn test_sample_input() {
        let sample_nums = parse_input(SAMPLE_INPUT);

        let base_10s: Vec<isize> = sample_nums.iter.map(|sn| sn.base10()).collect();
        assert_eq!(base_10s, vec![1747, 906, 198, 11, 201, 31, 1257, 32, 353, 107, 7, 3, 37]);

        let snafu_sum: Snafu = sample_nums.iter().sum();
        assert_eq!(snafu_sum.base10(), 4890);
        assert_eq!(snafu_sum.snafu(), "2=-1=0".to_string());
    }
}
