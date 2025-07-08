pub struct Binary {}

impl Binary {
    pub fn binary_search(list: &Vec<i32>, target: i32, start: &mut i32, end: &mut i32) -> i32 {
        let mut mid;
        while *start < *end {
            mid = *start as usize + ((*end as usize - *start as usize) / 2 as usize);
            if list[mid] == target {
                return mid as i32;
            } else if list[mid] > target {
                *end = list[mid];
            } else {
                *start = list[mid];
            }
        }
        -1
    }
}