//R1(100%, 47.54%);
//R2(100%, 24.12%);
//R3(100%, 47.54%);

//Total Score: 39.73 

use std::usize;

fn main() {
    println!("Hello, world!");
}

fn length_of_longest_substring(s: String) -> i32 {
    if s.len() == 0 {
        return 0;
    }

    let mut ansr = i32::min_value();
    let mut grapheme: Vec<bool> = vec![false; 128];
    let mut count: i32 = 0;
    let copy: Vec<char> = s.clone().chars().collect();
    let mut left: usize = 0;

    for letter in s.chars() {
        let idx = u32::from(letter) as usize;
        if grapheme[idx] == true {
            while copy[left] != letter {
                let idx2 = u32::from(copy[left]) as usize;
                grapheme[idx2] = false;
                count -= 1;
                left += 1;
            }
            left += 1;
        } else {
            grapheme[idx] = true;
            count += 1;
        }

        ansr = std::cmp::max(ansr, count);
    }

    ansr
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn case1() {
        assert_eq!(3, length_of_longest_substring("abcabcbb".to_string()))
    }

    #[test]
    pub fn case2() {
        assert_eq!(1, length_of_longest_substring("bbbbb".to_string()))
    }

    #[test]
    pub fn case3() {
        assert_eq!(3, length_of_longest_substring("pwwkew".to_string()))
    }
}
