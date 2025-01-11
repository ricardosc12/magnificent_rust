use std::{thread, time::Duration};

#[derive(Debug)]
struct Wrappe32(*mut i32);

impl Wrappe32 {
    fn get_value(&self) -> i32 {
        unsafe {
            return *self.0;
        }
    }

    fn change_value(&mut self) {
        unsafe {
            *self.0 = 20;
        }
    }
}

unsafe impl Send for Wrappe32 {}

fn main() {
    let mut teste = 10;

    let mut ref_teste = Wrappe32(&mut teste as *mut i32);

    let mut handles = vec![];

    let handle = thread::spawn(move || {
        println!("Value: {:?}", ref_teste.get_value());
        ref_teste.change_value();
        println!("Thread Changed: {:?}", ref_teste.get_value());
    });

    handles.push(handle);

    thread::sleep(Duration::from_secs(1));

    println!("Teste: {}", teste);

    for handle in handles {
        handle.join().unwrap();
    }
}
