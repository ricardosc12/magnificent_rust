
#[derive(Debug)]
struct Matriz {
    valor: u32,
    init: bool,
    right: Option<Box<Matriz>>,
    down: Option<Box<Matriz>>
}

impl Matriz {
    fn init(i: u32, j:u32) -> Option<Box<Matriz>> {

        let mut matriz = Some(Box::new(Matriz {
            valor: 10,
            right: None,
            down: None,
            init: true,
        }));

        let mut next_down = &mut matriz.as_mut().unwrap().down;

        for _ in 0..j {
            *next_down = Some(Box::new(Matriz {
                valor: 0,
                init: false,
                right: None,
                down: None,
            }));
            next_down = &mut next_down.as_mut().unwrap().down;
            
        }

        next_down = &mut matriz;

        let mut next_right = &mut matriz.as_mut().unwrap().right;

        for _ in 0..i {
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

    #[derive(Debug)]
    struct List {
        value: u32,
        next: Option<Box<List>>
    }

    impl List {
        fn init() -> Option<Box<List>> {
            let mut list = Some(Box::new(List{
                value: 0,
                next: None
            }));

            let mut next = &mut list.as_mut().unwrap().next;
            
            for i in 0..2 {
                *next = Some(Box::new(List{
                    value: i+1,
                    next: None
                }));
                
                next = &mut next.as_mut().unwrap().next;
            }

            next = &mut list;

            list
        }

        fn print(self) {
            print!("{} ", self.value);
            if self.next.is_some(){
                self.next.unwrap().print();
            }
        }

    }

    fn main() {
        // let matriz = Matriz::init(1,1);

        let list = List::init().unwrap();

        // matriz.insert(20);
        // matriz.insert(40);
        list.print();
        // println!("{:?}",list.unwrap());

        // println!("Matriz: {:?}",matriz);
        // println!("Matriz: {:?}",matriz.unwrap().down.unwrap().down);
        // matriz.print();
    }
