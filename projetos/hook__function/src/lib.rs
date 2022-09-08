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
use std::arch::asm;
use winapi::um::memoryapi::VirtualProtect;
use libc::{c_void};
use std::slice;
use ilhook::x86::{Hooker, HookType, Registers, CallbackOption, HookFlags};
use std::{thread, time::Duration};
use std::mem::transmute;

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

// unsafe fn hook(to_hook: DWORD, our_function: &mut c_void) {

//     let mut cur_protection: DWORD = 0;

//     VirtualProtect(to_hook as LPVOID, 7, 0x40, &mut cur_protection);

//     to_hook[0] = 0xE9;
    
// }

unsafe extern "cdecl" fn on_check_sn(reg: *mut Registers,_: usize) {
    println!("Value: {}", (*reg).eax);
}


fn hook_function() {

    unsafe { 
        
    consoleapi::AllocConsole();

    let mut to_hook: DWORD = 0x40147D;
    
    let mut cur_protection: DWORD = 0;

    VirtualProtect(to_hook as LPVOID, 7, 0x40, &mut cur_protection);
    

    let buf =  slice::from_raw_parts_mut(to_hook as *mut u8, 7);

    for x in 0..7 {
        buf[x] = 0x90;
    }

    let relative_addr: DWORD = our_funct as DWORD - to_hook as DWORD - 5;

    buf[0] = 0xE9;

    let bytes: [u8; 4] = [0x53,0xF8,0x10,0x10];
    // buf[1] = 0xD8;

    // let bytes = relative_addr.to_le_bytes();

    for x in 1..5 {
        buf[x] = bytes[x];
    }

    // buf[1..5].copy_from_slice(0x401484u32.to_le_bytes());

	// // Hook((void*)hookAddress, ourFunct, hookLength);

    let mut temp: DWORD = 0;

	VirtualProtect(to_hook as LPVOID, 7, cur_protection, &mut temp);




    println!(" hooked ! ");


    println!(" exit ");

    };

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