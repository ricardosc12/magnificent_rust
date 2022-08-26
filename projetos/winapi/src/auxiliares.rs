use std::string::String;
use std::io::{stdin, stdout, Read, Write};

pub fn cmp_array_string(string1:&str, array:&[i8]) -> u8 {
    let mut name = String::from("");
    for n in array {
        let new = *n as u8;
        if *n==0 {
            break;
        }
        name.push(new as char);
    }

    let back = if name == string1 { 1 } else if name == "?" { 2 } else { 0 };

    // println!("Array: {:?}",array);
    // print!("");
    
    drop(name);
    back
}


pub fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}
