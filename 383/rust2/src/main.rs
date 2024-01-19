use std::{collections::HashMap, usize};

fn main() {}

fn can_construct(ransom_note: String, magazine: String) -> bool {
    let mut freq = [0; 26];

    for c in magazine.chars() {
        let idx = u32::from(c) - 97;
        freq[idx as usize] += 1;
    }

    for c in ransom_note.chars() {
        let idx = u32::from(c) - 97;
        freq[idx as usize] -= 1;

        if freq[idx as usize] < 0 {
            return false
        }
    }

    true
}

#[cfg(test)]
pub mod tests {
    use crate::*;

    #[test]
    pub fn case1() {
        let ansr = can_construct("a".to_owned(), "b".to_owned());

        assert_eq!(ansr, false)
    }

    #[test]
    pub fn case2() {
        let ansr = can_construct("aa".to_owned(), "bb".to_owned());

        assert_eq!(ansr, false)
    }

    #[test]
    pub fn case3() {
        let ansr = can_construct("aa".to_owned(), "aab".to_owned());

        assert_eq!(ansr, true)
    }
}
