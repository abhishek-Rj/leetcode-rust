pub struct Solution {}

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let mut start: i32 = 1;
        let mut end: i32 = x;
        let mut mid;
        let mut ans = 0;
        while start <= end {
            mid = start + ((end - start ) / 2 );
            let square=  mid as i64*mid as i64;
            if square == x as i64 {
                return mid as i32;
            } else if square < x as i64 {
                ans = mid;
                start = mid as i32+ 1;
            } else {
                end = mid as i32 - 1;
            }
        }
        ans as i32
    }
}