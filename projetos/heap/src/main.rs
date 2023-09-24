struct Node {
    value: i32,
    next: Option<Box<Node>>,
}
impl Node {
    fn init(value: i32) -> Node {
        Node {
            value: value,
            next: None,
        }
    }
    fn insert(&mut self, value: i32) {
        if value >= self.value {
            if let Some(mut next) = self.next.take() {
                next.insert(value);
                self.next = Some(next);
            } else {
                self.next = Some(Box::new(Node { value, next: None }));
            }
        } else {
            let mut new_node = Box::new(Node::init(self.value));
            new_node.next = self.next.take();
            self.value = value;
            self.next = Some(new_node);
        }
    }
    fn print_node(self) {
        println!("Value: {}", self.value);
        if self.next.is_some() {
            self.next.unwrap().print_node();
        }
    }
}

fn main() {
    let mut node = Node::init(5);

    node.insert(1);
    node.insert(50);
    node.insert(10);
    node.insert(2);
    node.insert(1);

    node.print_node();
}
