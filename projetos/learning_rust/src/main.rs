use std::io;

fn main() {


    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Erro ao ler string");

    println!("-> {input}");

    let x:i32 = 2;
    let y:i32 = 2;

    println!("x = {x} and y + 2 = {}", y + 2);
}
