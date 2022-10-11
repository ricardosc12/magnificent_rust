
#[derive(Debug)]
struct List {
    valor: u32,
    next: Option<Box<List>>
}

impl List {
    fn init(value:u32) -> List {
        List {
            valor: value,
            next: None,
        }
    }
    fn insert(&mut self, valor:u32){
        if self.next.is_none() {
            self.next = Some(Box::new(List {
                valor: valor,
                next:None
            }));
        }
        else {
            self.next.as_mut().unwrap().insert(valor);
        }
    }
    fn print(self){
        println!("Valor: {:?}",self.valor);
        if self.next.is_some() {
            self.next.unwrap().print();
        }
    }
}

fn main() {
    let mut matriz = List::init(5);

    matriz.insert(20);
    matriz.insert(40);

    matriz.print();
}
