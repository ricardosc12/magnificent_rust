use std::arch::asm;
use std::{thread, time::Duration};
//Compilar em diferente target para remover erro
//cargo build --target=i686-pc-windows-msvc

extern "C" fn our_funct() {
    unsafe {
        asm!(
            "add eax, eax",
            "add eax, eax",
            "add eax, eax",
            "ret",
        );
    }
}

fn main() {

    let relatiave_address = unsafe { std::mem::transmute::<_, u32>(&our_funct) } ;

    println!("address: {:?}",relatiave_address);

    let mut num: i32 = 0;

    loop {
        // println!("{:?}",num);
        num = num + 1;

        thread::sleep(Duration::from_millis(1000));
    }

}
