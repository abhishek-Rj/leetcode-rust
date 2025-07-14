mod bubble_sort;
mod sum_zero;
mod maximum_bucket;
mod add_two_number2;
mod longest_sequence3;
mod median_array4;
mod longest_palindromic_string5;
mod zig_zag6;
mod container11;
mod sqrt69;
mod binarySearch;
mod repeating_character;
mod string_to_atoi8;
mod regular_expression10;
mod valid_anagram242;
mod unique_character387;
// use sum_zero::Solution;
// use maximum_bucket::Solution2;
// use add_two_number2::Solution;
// use longest_sequence3::Solution3;
// use crate::add_two_number2::ListNode;
// use median_array4::Solution;
// use zig_zag6::Solution;
// use container11::Solution;
// use rand::Rng;
// use string_to_atoi8::Solution;
// use regular_expression10::Solution;
// use valid_anagram242::Solution;
use unique_character387::Solution;
// use sqrt69::Solution;
// use binarySearch::Binary;
// use repeating_character::RepeatingCharacter;

fn main() {
    let s = String::from("rat");
    println!("{:?}", Solution::first_uniq_char(s));
    // let s = 1;
    // println!("{:?}", Solution::my_sqrt(s));
    // let list = vec![1,2,3,4,5,6,7,8];
    // let mut start = 0;
    // let mut end = ( list.len() - 1 ) as i32;
    // println!("{:?}",Binary::binary_search(&list, 7, &mut start, &mut end));
    // println!("{:?}", Solution::max_area(list));
    // let s = String::from("10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000522545459");
    // println!("{:?}", Solution::my_atoi(s));
    // let s = String::fro("0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000");
    // println!("{:?}", Solution::longest_palindrome(s));
    // println!("{:?}", Solution::if_palidrome(&s));
    // let number = 5;
    // let vec = vec![1,8,6,2,5,4,8,3,7];
    // let result = Solution2::max_area(vec);
    // println!("{:?}", result);
    // let  a_number = Box::new(43);
    // println!("This is adderss of a_address on stack, {:p}", &a_number);
    // println!("And this pointers a_address pointing at, {:p}", a_number.as_ref());
    // let option_value = Some(a_number);
    // match option_value.as_ref() {
    //     Some(b) => {
    //         println!("Option<Box<i32>> address meaning, adress of option_value is: {:p}", b);
    //         println!("Actual head address inside Box, {:p}", b.as_ref())
    //     }
    //     None => panic!()
    // }
    // println!("This is pointer option_value pointing at, {:p}", &option_value);

    // let l1 = ListNode::from_vec(vec![2,4,9]);
    // let l2 = ListNode::from_vec(vec![5,6,4,9]);
    // Solution::add_two_numbers(l1, l2);
    // let s = String::from("pwwkew");
    // println!("{}", Solution3::length_of_longest_substring(s));
    // let nums1 = vec![1,3];
    // let nums2 = vec![2, 4];
    // println!("{:?}", Solution::find_median_sorted_arrays(nums1, nums2));
}