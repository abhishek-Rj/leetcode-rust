pub struct Solution {}

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut list: Vec<char> = s.chars().collect();
        let ascii_number_list: Vec<i32> = vec![48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59];
        while list[0] == ' ' {
            list.remove(0);
        }
        let min: i64 = -2147483648;
        let max: i64 = 2147483647;
        let mut ans: Vec<i32> = vec![];
        let mut ans_num: i64  = 0;
        let mut if_negative = 1;
        let mut count = 0;
        for i in 0..list.len() {
            let chr = list[i];
            if i != 0 && !ascii_number_list.contains(&(chr as i32)) {
                break;
            }
            if i == count && chr == '-'{
                if_negative = -1;
                continue;
            } 
            if chr == ' ' {
                count += 1;
                continue;
            }
            if ascii_number_list.contains(&(chr as i32)) {
                ans.push(chr as i32 - 48);
            }
        }
        println!("{:?}", ans);
        let mut digit: i64= 1;
        for &i in ans.iter().rev() {
            ans_num = i as i64 * digit as i64 + ans_num;
            digit *= 10;
        } 
        ans_num = ans_num * if_negative;
        if max < ans_num {
            return max as i32;
        } else if min > ans_num {
            return min as i32;
        } else {
            ans_num as i32
        }
        
    }
}