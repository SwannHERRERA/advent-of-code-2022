use std::collections::HashSet;

pub fn get_hashset_of_str(subject: &str) -> HashSet<char> {
    let mut set: HashSet<char> = HashSet::new();
    for item in subject.chars() {
        set.insert(item);
    }
    set
}

pub fn calculate_letter_priority(letter: &char) -> u32 {
    const UPPERCASE_A: u8 = b'A';
    const LOWERCASE_A: u8 = b'a';
    const UPPERCASE_OFFSET: u8 = 26;

    let ascii_value = *letter as u8;
    let priority: u32 = match letter {
        'a'..='z' => (ascii_value - LOWERCASE_A + 1).into(),
        'A'..='Z' => (ascii_value - UPPERCASE_A + 1 + UPPERCASE_OFFSET).into(),
        _ => unreachable!(),
    };
    priority
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_hashset_of_part1() {
        const PART_1: &str = "vPwwTWBwg";
        let set = get_hashset_of_str(PART_1);
        assert!(set.contains(&'v'));
        assert!(set.contains(&'P'));
        assert!(set.contains(&'w'));
        assert!(set.contains(&'T'));
        assert!(set.contains(&'W'));
        assert!(set.contains(&'B'));
        assert!(set.contains(&'g'));
        assert_eq!(set.contains(&'y'), false);
        assert_eq!(set.len(), 7);
    }
}
