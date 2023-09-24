
struct LinkedList {
    value: u32,
    next: Option<Box<LinkedList>>,
}
impl LinkedList {
    fn create(value: u32) -> LinkedList {
        LinkedList {
            value: value,
            next: None,
        }
    }

    fn insert(&mut self, value: u32) {
        if self.next.is_none() {
            self.next = Some(Box::new(LinkedList { value: value, next: None }));
        }
        else {
            self.next.as_mut().unwrap().insert(value);
        }
    }

    fn print(&self) {
        print!("{} ", self.value);

        if self.next.is_some() {
            self.next.as_ref().unwrap().print();
        }
    }
}

fn main() {
    let mut linked_list = LinkedList::create(10);
    linked_list.insert(20);
    linked_list.print();
}
