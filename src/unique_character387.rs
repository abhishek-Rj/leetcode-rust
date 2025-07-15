use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut record: HashMap<char, usize> = HashMap::new();
        for (index, char) in s.chars().enumerate() {
            if record.contains_key(&char) {
                *record.get_mut(&char).unwrap() += 1;
            } else {
                record.insert(char, 1);
            }
        }        
        for (index, chr ) in s.chars().enumerate() {
            if record[&chr] == 1 {
                return index as i32;
            }
        }
        -1 
    }
}