pub struct Solution2;

impl Solution2 {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max = 0;
        for i in 0..height.len() {
            for j in 0..height.len() {
                if i == j {
                    continue;
                }
                let smallest_length =  if height[i] > height[j] { height[j] } else { height[i] };
                let width_difference = i.abs_diff(j);
                let current_max = smallest_length * width_difference as i32;
                if current_max > max {
                    max = current_max;
                }
            }
        }

        max
    }
}
