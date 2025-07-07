pub fn bubble_sort(list: &mut Vec<i32>) {
    let vec_size = list.len();
    for i in 0..vec_size - 1 {
        for j in 0..vec_size - i - 1 {
            if list[j] > list[j + 1] {
                let temp = list[j];
                list[j] = list[j + 1];
                list[j + 1] = temp;
            }
        }
    }
}