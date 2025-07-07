use std::{collections::HashMap, hash::Hash};

pub struct Solution {}

impl Solution {
    pub fn if_palindrome(s: &str) -> bool {
        s.chars().eq(s.chars().rev())
    }
 
    pub fn longest_palindrome(s: String) -> String {
        let mut new_string: String = "".to_string();
        let mut map: HashMap<String,i32> = HashMap::new();
        if s.len() == 1 {
            return s;
        }
        let chars: Vec<char> = s.chars().collect();
        for (tt, ch ) in s.chars().enumerate() {
            for ttt in tt+1..chars.len() {
                if chars[ttt] == ch {
                    new_string = chars[tt..=ttt].iter().collect();
                    if Solution::if_palindrome(&new_string) {
                        map.insert(new_string.clone(), new_string.len() as i32);               
                    }
                    new_string.clear();
                }
            }
            map.insert(ch.to_string(), 1);
        }

        let mut max_val:i32 = 0;
        let mut ans: String ="".to_string();
        for (k, &v) in &map {
            if v > max_val {
                max_val = v;
                ans = k.to_string();
            }
        }
        println!("{:?}", map);
        ans
    }
}