// Métodos disponíveis de build em https://github.com/ricardosc12/magnificent_rust/blob/main/projetos/hook__function/src/lib.rs
// cargo build --lib --target=i686-pc-windows-msvc

use std::arch::asm;
use std::{thread, time::Duration};
use winapi::um::winuser::GetAsyncKeyState;
use winapi::shared::minwindef;
use winapi::shared::minwindef::{BOOL, DWORD, HINSTANCE, LPVOID};
use winapi::um::consoleapi;

mod keyboard;
use crate::keyboard::Key;



unsafe fn read_from_adress(adress:u32) -> i32 {
    let mut data = 0;
    asm!{
        "mov {}, [{}]", 
        out(reg) data,
        in(reg) adress, 
    };
    return data;
}

fn hook_function() {
    println!("injected !!");
    
    unsafe {
        thread::spawn(|| {

            let value = 0x0061FF0C as *mut i32; 

            let mut home_key = Key::init(0x24);

            consoleapi::AllocConsole();

            loop {
                if GetAsyncKeyState(0x23) != 0 { //end key
                    break;
                }
                
                else if home_key.check() {
                    // var = read_from_adress(0x0061FF0C);
                    *value = 2;
                    println!("Var: -- {:?}",*value);
                }
                thread::sleep(Duration::from_millis(50));
            }
            println!("exiting...");
        });
    }


    
}

#[no_mangle]
#[allow(non_snake_case, unused_variables)]

extern "system" fn DllMain(
    dll_module: HINSTANCE,
    call_reason: DWORD,
    reserved: LPVOID)
    -> BOOL
{
    const DLL_PROCESS_ATTACH: DWORD = 1;
    const DLL_PROCESS_DETACH: DWORD = 0;

    match call_reason {
        DLL_PROCESS_ATTACH => hook_function(),
        DLL_PROCESS_DETACH => (),
        _ => ()
    }
    minwindef::TRUE
}

// fn main(){
//     hook_function()
// }


