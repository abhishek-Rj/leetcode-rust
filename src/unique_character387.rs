use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn first_uniq_char(s: String){
        let mut record: HashMap<char, usize> = HashMap::new();
        for (index, char) in s.chars().enumerate() {
            if record.contains_key(&char) {
                *record.get_mut(&char).unwrap() += 1;
            } else {
                record.insert(char, 1);
            }
        }        
        println!("{:?}", record);
    }
}