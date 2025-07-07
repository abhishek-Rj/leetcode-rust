pub struct Solution {}
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut nums3: Vec<i32> = Vec::new();
        nums3.extend(nums1);
        nums3.extend(nums2);
        nums3.sort();
        if nums3.len() % 2 != 0 {
            return nums3[nums3.len()/2] as f64;
        } else {
            return ((nums3[nums3.len()/2] as f64 + (nums3[nums3.len()/2 - 1]) as f64) / 2 as f64) as f64;
        }
    }
}