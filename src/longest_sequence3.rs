use std::collections::HashMap;
pub struct Solution3 {}

impl Solution3 {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut list: Vec<char> = Vec::new();
        let mut max = 0;
        let string_size = s.len();
        if s == " " || s.len() == 1 {
            return 1;
        }
        for i in 0..string_size {
            if let Some(tt) = s.chars().nth(i) {
                if !list.contains(&tt) {
                    list.push(tt);
                } else {
                    if let Some(ttt) = list.iter().position(|&x| x == tt) {
                        list.drain(0..=ttt);
                    }
                    list.push(tt);
                }
                if list.len() as i32 > max {
                    max = list.len() as i32;
                }
            }    
        }
        max
    }
}