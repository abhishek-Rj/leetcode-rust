pub struct Solution {}

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut ss: Vec<char> = s.chars().collect();
        let mut tt: Vec<char> = t.chars().collect();
        ss.sort();
        tt.sort();
        return ss == tt;
    }
}