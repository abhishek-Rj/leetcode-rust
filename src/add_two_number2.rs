// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    pub fn from_vec(vec: Vec<i32>) -> Option<Box<ListNode>> {
        let mut current = None;
        for &v in vec.iter().rev() {
            let mut node = Box::new(ListNode::new(v));
            node.next = current;
            current = Some(node);
        }
        current
    }

    pub fn print_list(head: &Option<Box<ListNode>>) {
        let mut current = head.as_ref();
        while let Some(node) = current {
            print!("{} -> ", node.val);
            current = node.next.as_ref();
        }
        println!("None");
    }
}

pub struct Solution {}
impl Solution {
    pub fn add_two_numbers( l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let l3: Option<Box<ListNode>>;
        let mut list1: Vec<i32> = Vec::new();
        let mut list2: Vec<i32> = Vec::new();
        let mut p1 = l1.as_ref();
        let mut p2 = l2.as_ref();

        while let Some(tt) = p1 {
            let current_value = tt.as_ref().val;
            list1.push(current_value);
            p1 = tt.as_ref().next.as_ref();
        }
        while let Some(tt) = p2 {
            let current_value = tt.as_ref().val;
            list2.push(current_value);
            p2 = tt.as_ref().next.as_ref();
        }
        let length_list1 = list1.len();
        let length_list2 = list2.len();

        if list1.len() > list2.len() {
            let diff = length_list1 - length_list2;
            for _ in 1..=diff {
                list2.push(0);
            }
        } else if list1.len() < list2.len() {
            let diff = length_list2 - length_list1;
            for _ in 1..=diff {
                list1.push(0);
            }
        }

        let mut list3:  Vec<i32> = Vec::new();
        let mut carry = 0;
        let max_length = if list1.len() >= list2.len() {list1.len()} else {list2.len()};
        for i in 0..max_length{
            let mut sum;
            if carry == 1 {
                sum = list1[i] + list2[i] + carry;
                carry = 0;
            } else {
                sum = list1[i] + list2[i];
            }
            if sum >= 10 {
                sum = sum % 10;
                carry = 1;
            }
            list3.push(sum);
            if (i == max_length - 1) && (carry == 1) {
                list3.push(1);
            }
        }
        println!("{:?}, {:?}, {:?}", list1, list2, list3);
        let mut current: Option<Box<ListNode>> = None;
        for &i in list3.iter().rev() {
            let mut create_node = Box::new(ListNode::new(i));
            create_node.next = current;
            current = Some(create_node);
        }
        current
    }
}