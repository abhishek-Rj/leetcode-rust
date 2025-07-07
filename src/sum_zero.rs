pub struct Solution;

impl Solution {
    pub fn sum_zero(n: i32) ->  Vec<i32> {
        let mut vec = Vec::new();
        for i in 0..n {
            vec.push(((i * 2) - n) + 1)
        }
        vec
    }
}