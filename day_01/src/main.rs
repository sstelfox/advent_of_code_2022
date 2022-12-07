const INPUT_DATA: &'static [u8] = include_bytes!("../data/input");

fn count_elf_calories(data: &[u8]) -> Vec<(usize, usize)> {
    let data = std::str::from_utf8(data).unwrap();

    let mut all_elves = vec![];

    // The elfs are one-indexed
    let mut current_elf = 1;
    let mut current_count = 0;

    for line in data.lines() {
        if line.trim().is_empty() {
            all_elves.push((current_elf, current_count));

            current_count = 0;
            current_elf += 1;

            continue;
        }

        current_count += line.trim().parse::<usize>().unwrap();
    }

    all_elves.push((current_elf, current_count));
    all_elves.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    all_elves
}

fn main() {
    let all_elves = count_elf_calories(INPUT_DATA);

    let result1: Option<&(usize, usize)> = all_elves.iter().rev().last();
    println!("{:?}", result1);

    let result2: Vec<&(usize, usize)> = all_elves.iter().rev().take(3).collect();
    println!("{:?}", result2.iter().map(|(_, count)| count).sum::<usize>());
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &'static [u8] = include_bytes!("../data/sample");

    #[test]
    fn test_sample_input() {
        let elf_counts = count_elf_calories(SAMPLE_INPUT);

        assert_eq!(
            elf_counts,
            vec![(2, 4000), (1, 6000), (5, 10000), (3, 11000), (4, 24000)]
        );
    }
}
