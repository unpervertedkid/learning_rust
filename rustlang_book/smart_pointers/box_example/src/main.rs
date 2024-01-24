#[derive(Debug)]
struct LinkedListNode {
    value: i32,
    // We use Box here because it is impossible to know the size
    // of the next node at compile time. Box is a pointer to a
    // heap-allocated value.
    next: Option<Box<LinkedListNode>>,
}

impl LinkedListNode {
    fn new(value: i32) -> LinkedListNode {
        LinkedListNode {
            value,
            next: None,
        }
    }
    
    fn append(&mut self, value: i32) {
        let mut current_node = self;
        loop {
            match current_node.next {
                Some(ref mut next_node) => current_node = next_node,
                None => {
                    current_node.next = Some(Box::new(LinkedListNode::new(value)));
                    break;
                }
            }
        }
    }

    fn prepend(&mut self, value: i32) {
        let mut new_head = LinkedListNode::new(value);
        std::mem::swap(&mut new_head.next, &mut self.next);
        std::mem::swap(self, &mut new_head);
    }
}


fn main() {
    let head = LinkedListNode {
        value: 1,
        next: Some(Box::new(LinkedListNode {
            value: 2,
            next: Some(Box::new(LinkedListNode {
                value: 3,
                next: None,
            })),
        })),
    };
    
    print_list(&head);
}

fn print_list(head: &LinkedListNode) {
    let mut current_node = head;
    loop {
        print!("{}", current_node.value);
        match &current_node.next {
            Some(next_node) => {
                print!(" -> ");
                current_node = next_node;
            },
            None => break,
        }
    }
    println!();
}