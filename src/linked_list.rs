struct Node {
    data: i32,
    next: Option<Box<Node>>
}

struct LinkedList {
    head_pointer: Option<Box<Node>>,
}

impl LinkedList {
    pub fn new () -> Self {
        LinkedList { head_pointer: None }
    }

    pub fn add_data (&mut self, data: i32) {
        let new_node = Box::new(Node{
            data: data,
            next: self.head_pointer.take()
        });
        self.head_pointer = Some(new_node);
    }

    pub fn view_data (&self) {
        let mut curr_poin= self.head_pointer.as_ref();
        while let Some(node) = curr_poin {
            println!("{}", node.data);
            curr_poin = node.next.as_ref();
        }
    }
}

fn main() {
    let mut obj = LinkedList::new();
    obj.add_data(23);
    obj.add_data(12);
    obj.add_data(3);
    obj.view_data();
}   
