use std::mem;

struct BinaryTree {
    node: Option<Box<Node>>,
}
#[derive(Clone)]
struct Node {
    value: u32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}
impl Node {
    fn create(value: u32) -> Node {
        Node {
            value: value,
            left: None,
            right: None,
        }
    }
    fn insert(&mut self, value: u32) {
        if value < self.value {
            if self.left.is_none() {
                self.left = Some(Box::new(Node::create(value)));
            } else {
                self.left.as_mut().unwrap().insert(value);
            }
        } else {
            if self.right.is_none() {
                self.right = Some(Box::new(Node::create(value)));
            } else {
                self.right.as_mut().unwrap().insert(value);
            }
        }
    }

    fn print_inoder(&self) {
        if self.left.is_some() {
            self.left.as_ref().unwrap().print_inoder();
        }

        print!("{} ", self.value);

        if self.right.is_some() {
            self.right.as_ref().unwrap().print_inoder();
        }
    }

    fn delete(&mut self, value: u32) -> Option<Box<Node>> {
        if self.value == value {
            if self.left.is_none() {
                return self.right.clone()
            }
            else if self.right.is_none() {
                return self.left.clone()
            }
            else {
                let mut node = self.left.clone();

                while node.as_ref().unwrap().right.is_some() {
                    node = node.unwrap().right.clone();
                }

                self.value = node.as_ref().unwrap().value;
                node.as_mut().take();

                return Some(Box::new(self.clone()));
            }
        }
        else if value < self.value {
            return self.left.as_mut().unwrap().delete(value);
        }
        else {
            return self.right.as_mut().unwrap().delete(value);
        }
    }
}
impl BinaryTree {
    fn create(value: u32) -> BinaryTree {
        let node = Node {
            value: value,
            left: None,
            right: None,
        };
        BinaryTree {
            node: Some(Box::new(node)),
        }
    }

    fn insert(&mut self, value: u32) {
        if self.node.is_none() {
            *self = BinaryTree::create(value)
        } else {
            self.node.as_mut().unwrap().insert(value);
        }
    }

    fn delete(&mut self, value: u32) {
        if self.node.is_some() {
            // self.node = self.node.as_mut().unwrap().delete(value)
            self.node.as_mut().unwrap().delete(value);
        }
    }

    fn print_inoder(&self) {
        if self.node.is_none() {
            println!("Árvore vazia!");
        } else {
            self.node.as_ref().unwrap().print_inoder();
        }
        println!("");
    }
}

fn main() {
    let mut binary_tree = BinaryTree::create(10);

    // binary_tree.insert(5);
    binary_tree.insert(15);
    // binary_tree.insert(3);
    // binary_tree.insert(7);
    // binary_tree.insert(11);
    binary_tree.insert(17);
    binary_tree.insert(14);

    binary_tree.print_inoder();

    binary_tree.delete(15);

    binary_tree.print_inoder();
}
