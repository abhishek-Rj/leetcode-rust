pub struct Solution {}
impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut new_string = s.trim().to_string();
        let ascii_number_list: Vec<i32> = vec![48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59];
        let mut is_negative = 1;
        if new_string.is_empty() {
            return 0;
        }
        if let Some(b) = new_string.chars().nth(0) {
            if b == '-' {
                is_negative = -1;
                new_string.remove(0);
            }
            if b == '+' {
                new_string.remove(0);
            }
        } 
        if new_string.is_empty() {
            return 0;
        }
        for (index, char) in new_string.chars().enumerate() {
            let ascii_val = char as i32;
            if !ascii_number_list.contains(&ascii_val) {
                new_string = new_string[0..index].to_string();
                break;
            }
        }
        if new_string.is_empty() {
            return 0
        }
        println!("{:?}", new_string);
        let mut char_vec: Vec<char> = new_string.chars().collect();
        if char_vec.is_empty() {
            return 0;
        } 
        while !char_vec.is_empty() && char_vec[0] == '0' {
            char_vec.remove(0);
        }
        println!("{:?}", char_vec);
        let mut digit: i64 = 1;
        let mut ans: i64 = 0;
        for i in char_vec.iter().rev() {
            let char_value = *i as i32 - 48;
            ans = char_value as i64 * digit + ans;
            if ans * is_negative > i32::MAX as i64 {
                return i32::MAX
            } else if ans * is_negative < i32::MIN as i64 {
                return i32::MIN
            }
            digit = match digit.checked_mul(10) {
                Some(d) => d,
                None => return if is_negative == 1 { i32::MAX } else { i32::MIN },
            };
        }
        let result = ans * is_negative;
        println!("{:?}", result);
        result as i32
    }
}