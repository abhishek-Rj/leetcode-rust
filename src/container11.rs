pub struct Solution {}
impl Solution {
    //brute force
    // pub fn max_area(height: Vec<i32>) -> i32 {
    //     let mut max_area = 0;
    //     for i in 0..height.len()- 1 {
    //         for j in i+1..height.len() {
    //             let height_to_consider = if height[i] < height[j] { height[i] } else { height[j] };
    //             let area = height_to_consider * (i as i32 - j as i32).abs();
    //             max_area = max_area.max(area);
    //         }
    //     }
    //     max_area
    // }

    pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut left, mut right) = (0, height.len() - 1);
        let mut max_area = 0;

        while left < right {
            let height_to_consider = height[left].min(height[right]);
            let area = height_to_consider * (left as i32 - right as i32).abs();
            max_area = max_area.max(area);

            if height[left] > height[right] {
                right -= 1;
            } else {
                left += 1;
            }
        }
        max_area
    }
}