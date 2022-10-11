
#[derive(Debug)]
struct Matriz {
    valor: u32,
    init: bool,
    right: Option<Box<Matriz>>,
    down: Option<Box<Matriz>>
}

impl Matriz {
    fn init(i: u32, j:u32) -> Matriz {

        let mut matriz = Matriz {
            valor: 0,
            right: None,
            down: None,
            init: true,
        };

        let mut next_down = &mut matriz.down;
        let mut next_right = &mut matriz.right;

        for _ in 0..i {
            *next_down = Some(Box::new(Matriz {
                valor: 0,
                init: false,
                right: None,
                down: None,
            }));
            next_down = &mut next_down.as_mut().unwrap().down;
        }

        for _ in 0..j {
            *next_right = Some(Box::new(Matriz {
                valor: 0,
                init: false,
                right: None,
                down: None,
            }));
            next_right = &mut next_right.as_mut().unwrap().right;
        }

        matriz

    }
    // fn insert(&mut self, valor:u32) {
    //     if self.next.is_none() {
    //         self.next = Some(Box::new(Matriz {
    //             valor: valor,
    //             next: None
    //         }));
    //         return
    //     }
    //     self.next.as_mut().unwrap().insert(valor);
        
    // }
    // fn print(self){
    //     println!("Valor: {:?}",self.valor);
    //     if self.next.is_some() {
    //         self.next.unwrap().print();
    //     }
    // }
}

fn main() {
    let mut matriz = Matriz::init(1,1);

    // matriz.insert(20);
    // matriz.insert(40);

    println!("Matriz: {:?}",matriz);
    // matriz.print();
}
