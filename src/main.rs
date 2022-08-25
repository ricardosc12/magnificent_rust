use std::{thread::{self, JoinHandle}, time::Duration};
use std::sync::{Arc, Mutex};

//Bom aprendizado
// Em outras linguagens, criar threads que compartilham de um mesmo dado é fácil, mas como Rust tem uma tratativa de segurança rígida, este tipo de abordagem cria
// espaço para condições de corrida.
//Podemos resolver isso utilizando Mutex, ele faz com que uma thread só possa acessar esse dado usando lock, ou seja, estou utilizando este dado, e para as outras que pretendem
//devem esperar já que seus lock não permitiram, "só uma thread pode estar acessando aquele dado".
//Mas mesmo assim, ainda não poderiamos compartilhar esse dado com várias threads, como em outras LP, Thread em Rust não compartilham o mesmo espaço de endereçamento 
//do thread main
// Mas é possível permitir isso usando Arc. :) 

//Exemplo para compartilhar um vetor, cuja duas threads acesssam e o alteram, e uma apenas visualiza os dados.

fn main() {
    let vetor: Arc<Mutex<Vec<i32>>> = Arc::new(Mutex::new(Vec::new()));
    let mut handles: Vec<JoinHandle<()>> = vec![];
    
    {
        let vetor = Arc::clone(&vetor);
        let handle = thread::spawn(move || {
            loop {
                vetor.lock().unwrap().push(1);
                thread::sleep(Duration::from_millis(1000));
            }
        });
        handles.push(handle);
    }

    {
        let vetor = Arc::clone(&vetor);
        let handle = thread::spawn(move || {
            loop {
                vetor.lock().unwrap().push(2);
                thread::sleep(Duration::from_millis(2000));
            }
        });
        handles.push(handle);
    }

    {
        let handle = thread::spawn(move || {
            loop {
                println!("Vetor: {:?}",vetor.lock().unwrap());
                thread::sleep(Duration::from_millis(1000));
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

}
