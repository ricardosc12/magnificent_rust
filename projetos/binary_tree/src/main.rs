use std::cmp::Ordering::{Equal, Greater, Less};

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

    fn delete(node: *mut Option<Box<Node>>, value: u32) {
        unsafe {
            if (*node).is_some() {
                if value < (*node).as_ref().unwrap().value {
                    Node::delete(&mut (*node).as_mut().unwrap().left, value)
                } else if value > (*node).as_ref().unwrap().value {
                    Node::delete(&mut (*node).as_mut().unwrap().right, value)
                } else {
                    if (*node).as_ref().unwrap().left.is_none() {
                        (*node) = (*node).as_mut().unwrap().right.clone();
                    }
                    else if (*node).as_ref().unwrap().right.is_none() {
                        (*node) = (*node).as_mut().unwrap().left.clone();
                    }
                    else {
                        let mut suc = &mut (*node).as_mut().unwrap().right;

                        while (*suc).as_ref().unwrap().left.is_some() {
                            suc = &mut (*suc).as_mut().unwrap().left;
                        }

                        (*node).as_mut().unwrap().value = suc.as_ref().unwrap().value;

                        *suc = None;
                    }
                }
            }
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
            // self.node = self.node.as_mut().unwrap().delete(value);
            Node::delete(&mut self.node, value);
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

    // // binary_tree.insert(5);
    binary_tree.insert(8);
    // // binary_tree.insert(3);
    binary_tree.insert(11);
    binary_tree.insert(2);
    binary_tree.insert(9);
    // binary_tree.insert(18);
    // binary_tree.insert(11);
    // binary_tree.insert(14);

    binary_tree.print_inoder();

    binary_tree.delete(9);

    binary_tree.print_inoder();
}
