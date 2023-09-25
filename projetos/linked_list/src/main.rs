struct LinkedList {
    node: Option<Node>,
}
#[derive(Clone)]
struct Node {
    value: u32,
    next: Option<Box<Node>>,
}
impl Node {
    fn create(value: u32) -> Node {
        Node {
            value: value,
            next: None,
        }
    }
    fn insert(&mut self, value: u32) {
        if self.next.is_none() {
            self.next = Some(Box::new(Node {
                value: value,
                next: None,
            }));
        } else {
            self.next.as_mut().unwrap().insert(value);
        }
    }
    fn delete(&mut self, value: u32) {
        let mut current = self;

        while current.next.is_some() {
            if current.next.as_ref().unwrap().value == value {
                current.next = current.next.clone().unwrap().next;
            }
            else {
                current = current.next.as_mut().unwrap().as_mut();
            }
        }        
    }
    fn print(&self) {
        print!("{} ", self.value);

        if self.next.is_some() {
            self.next.as_ref().unwrap().print();
        }
    }
}
impl LinkedList {
    fn create(value: u32) -> LinkedList {
        LinkedList {
            node: Some(Node::create(value)),
        }
    }
    fn insert(&mut self, value: u32) {
        if self.node.is_none() {
            self.node = Some(Node {
                value: value,
                next: None,
            })
        } else {
            self.node.as_mut().unwrap().insert(value);
        }
    }
    fn delete(&mut self, value: u32) {
        if self.node.is_none() {
            println!("Empty list!");
        } else {
            if self.node.as_ref().unwrap().value == value {
                if self.node.as_ref().unwrap().next.is_some() {
                    self.node = Some(*self.node.as_ref().unwrap().next.as_ref().unwrap().clone());
                }
                else {
                    self.node = None;
                }
            }
            else {
                self.node.as_mut().unwrap().delete(value);
            }
        }
    }
    fn print(&self) {
        if self.node.is_some() {
            self.node.as_ref().unwrap().print();
        } else {
            println!("Empty list!");
        }
    }
}

fn main() {
    let mut linked_list = LinkedList::create(1);

    linked_list.insert(2);
    linked_list.insert(3);
    linked_list.delete(1);
    linked_list.delete(2);
    linked_list.delete(3);
    
    linked_list.print();
}
