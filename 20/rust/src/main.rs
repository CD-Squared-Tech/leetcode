//R1 (Speed: 100%, Memory: 88.08) = 88.08;
//R2 (Speed: 100%, Memory: 46.26) = 46.26;
//R3 (Speed: 100%: Memory: 46.26) = 46.26;
//FINAL SCORE: 60.2
fn main() {
    println!("Hello, world!");
}

pub fn is_valid(s: String) -> bool {
    let arr: [char; 6] = ['(', ')', '{', '}', '[', ']'];
    let mut stack: Vec<char> = Vec::new();

    for letter in s.chars() {
        match arr.iter().position(|x| x == &letter) {
            Some(y) => {
                if (y % 2) == 0 {
                    stack.push(arr[y + 1]);
                } else {
                    match stack.pop() {
                        Some(z) => {
                            if z != arr[y] {
                                return false;
                            }
                        }
                        None => {
                            if stack.len() == 0 {
                                return false;
                            }
                        }
                    }
                }
            }
            None => (),
        }
    }

    if stack.len() > 0 {
        return false;
    }

    true
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn case_1() {
        is_valid("()".into());
        assert_eq!(is_valid("()".into()), true)
    }

    #[test]
    pub fn case_2() {
        assert_eq!(is_valid("(){}[]".into()), true);
    }

    #[test]
    pub fn case_3() {
        println!("TEST 3");
        assert_eq!(is_valid("(]".into()), false);
    }
}
