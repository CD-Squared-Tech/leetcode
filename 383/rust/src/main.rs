use std::collections::HashMap;

fn main() {}

fn can_construct(ransom_note: String, magazine: String) -> bool {
    let mut map: HashMap<char, i32> = HashMap::new();
    let mut ansr: bool = true;

    magazine
        .chars()
        .into_iter()
        .for_each(|x| match map.get(&x) {
            Some(y) => {
                map.insert(x, y + 1);
            }
            None => {
                map.insert(x, 1);
            }
        });

    ransom_note
        .chars()
        .into_iter()
        .for_each(|x| match map.get(&x) {
            Some(y) => {
                if *y == 1 {
                    map.remove(&x);
                } else {
                    map.insert(x, y - 1);
                }
            }
            None => {
                ansr = false;
            }
        });

    ansr
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
