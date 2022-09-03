//Para construir uma lib dinâmica (dll) deve rodar cargo build --lib
//A dll gerada será com arquitetura padrão do sistema, caso queira alterar
//rustup target add i686-pc-windows-msvc   Exemplo para x86  -   irá baixar os binários para compilar em x86
//Por fim cargo build --lib --target=i686-pc-windows-msvc

// references: https://doc.rust-lang.org/reference/linkage.html, 
// https://gist.github.com/CoolOppo/67b452c125bb0db3212a9fbc44c84245,
// https://github.com/rkarp/rust-dll-demo/blob/master/src/lib.rs

#![cfg(windows)]

use winapi::shared::minwindef;
use winapi::shared::minwindef::{BOOL, DWORD, HINSTANCE, LPVOID};
use winapi::um::consoleapi;

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
        DLL_PROCESS_ATTACH => demo_init(),
        DLL_PROCESS_DETACH => (),
        _ => ()
    }
    minwindef::TRUE
}

fn demo_init() {
    unsafe { consoleapi::AllocConsole() };
    println!("Hello, world!");
}

