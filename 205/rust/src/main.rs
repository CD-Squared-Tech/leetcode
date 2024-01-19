use std::{char, usize};

fn main() {
    println!("Hello, world!");
}

pub fn is_isomorphic(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut morph: Vec<char> = vec![char::default(); 128];

    for (x, y) in s.chars().zip(t.chars()) {
        let idx = u32::from(x) as usize;
        if morph[idx] != char::default() && morph[idx] != y {
            return false;
        } else {
            morph[idx] = y;
        }
    }

    let h1: std::collections::HashSet<char> = s.chars().into_iter().collect();
    let h2: std::collections::HashSet<char> = t.chars().into_iter().collect();

    if h1.len() != h2.len() {
        return false;
    }

    true
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn case1() {
        assert_eq!(is_isomorphic("egg".to_string(), "add".to_string()), true)
    }

    #[test]
    pub fn case2() {
        assert_eq!(is_isomorphic("foo".to_string(), "bar".to_string()), false)
    }

    #[test]
    pub fn case3() {
        assert_eq!(
            is_isomorphic("paper".to_string(), "title".to_string()),
            true
        )
    }

    #[test]
    pub fn case4() {
        assert_eq!(is_isomorphic("badc".to_string(), "baba".to_string()), false)
    }

    #[test]
    pub fn case5() {
        assert_eq!(is_isomorphic("13".to_string(), "42".to_string()), true)
    }
}
