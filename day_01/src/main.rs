const INPUT1_DATA: &'static [u8] = include_bytes!("../data/input1");

fn count_elf_calories(data: &[u8]) -> Option<(usize, usize)> {
    let data = std::str::from_utf8(data).unwrap();

    // (id, count)
    let mut highest: Option<(usize, usize)> = None;

    let mut current_elf: usize = 0;
    let mut current_count: usize = 0;

    for line in data.lines() {
        if line.trim().is_empty() {
            if highest.is_none() || current_count > highest.unwrap().1 {
                highest = Some((current_elf, current_count));
                current_count = 0;
            }

            current_elf += 1;
            continue;
        }

        current_count += line.parse::<usize>().unwrap();
    }

    if current_count > highest.unwrap().1 {
        highest = Some((current_elf, current_count));
    }

    highest
}

fn main() {
    let result = count_elf_calories(INPUT1_DATA);
    println!("{:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &'static [u8] = include_bytes!("../data/sample");

    #[test]
    fn test_sample_input() {
        assert_eq!(count_elf_calories(SAMPLE_INPUT), Some((3, 24000)));
    }
}
