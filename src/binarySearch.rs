pub struct Binary {}

impl Binary {
    pub fn binary_search(list: Vec<i32>, target: i32, start: i32, end: i32) {
        let list_length = list.len();
        let mid = start + ((end - start) / 2);
        print!("{}", mid);
    }
}