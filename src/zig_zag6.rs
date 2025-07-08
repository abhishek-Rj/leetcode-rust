pub struct Solution {}

impl Solution {
    pub fn convert(s: String, num_rows: i32) {
        let string_length = s.len();
        println!("{}", string_length);
        let nums_of_matrix = (string_length as f64 / (num_rows as f64 - 1 as f64));
        println!("{:?}", nums_of_matrix);
    }
}