//Para construir uma lib dinâmica (dll) deve rodar cargo build --lib
//A dll gerada será com arquitetura padrão do sistema, caso queira alterar
//rustup target add i686-pc-windows-msvc   Exemplo para x86  -   irá baixar os binários para compilar em x86
//Por fim cargo build --lib --target=i686-pc-windows-msvc

// references: https://doc.rust-lang.org/reference/linkage.html, 
// https://gist.github.com/CoolOppo/67b452c125bb0db3212a9fbc44c84245,
// https://github.com/rkarp/rust-dll-demo/blob/master/src/lib.rs

//jmp tem 5 bytes
// E9 FF FF FF FF → E9 jmp instruction
//o resto deve ser o endereço relativo em ordem do menos significativo para o mais significativo
// exemplo 0x40147D → 0x401484
// 0x401484 - 0x40147D = 7 → -5 do jmp = 2
// #entender porque -5# ##tamanho da instrução de jump##
// 2 u32 em array de 4 bytes = [0,0,0,2]
// porque 4 ? jmp tem 5, -1 dele, sobra 4
// então fica E9 02 00 00 00 → menos significativo para mais

// Compilar em arquitetura 32bits
// cargo build --lib --target=i686-pc-windows-msvc

// Depois de algumas pesquisas, aparentemente Rust não usa constantes nos blocos assembly, diferente de C, então é necesário carregar o valor
// para um registrador, e usar este registrador, problemas:
// Ele faz isso movendo o valor para o registrador na primeira instrução - e se tivermos usando este registrador ? podemos passar out("eax") _, assim
// rust não utiliza o eax e utiliza outro, mas mesmo assim geraria problemas, pois e caso em alguma parte do código o outro registrador escolhido
// estivesse sendo usado.

#![cfg(windows)]

use winapi::shared::minwindef;
use winapi::shared::minwindef::{BOOL, DWORD, HINSTANCE, LPVOID};
use winapi::um::consoleapi;
use std::arch::{asm};
use winapi::um::memoryapi::VirtualProtect;
use libc::{c_void};
use std::slice;
use ilhook::x86::{Hooker, HookType, Registers, CallbackOption, HookFlags};
use std::{thread, time::Duration};
use std::mem::transmute;


// unsafe fn hook(to_hook: DWORD, our_function: &mut c_void) {

//     let mut cur_protection: DWORD = 0;

//     VirtualProtect(to_hook as LPVOID, 7, 0x40, &mut cur_protection);

//     to_hook[0] = 0xE9;
    
// }

// unsafe extern "cdecl" fn on_check_sn(reg: *mut Registers,_: usize) {
//     println!("Value: {}", (*reg).eax);
// }


fn hook_function() {

    let jump_back: DWORD = 0x40147D + 7;

    fn our_funct() {
        unsafe {
            asm!(
                "push eax",
                "add dword ptr [ebp-0x0C],02",
                "mov eax,[ebp-0x0C]",
                "ret",in("eax") 0x401484u32
                // "jmp {}", in(reg) 0x401484u32,
                // out("eax") _
            );
        }
    }

    unsafe { 
        
    consoleapi::AllocConsole();

    // let relatiave_address = unsafe { std::mem::transmute::<_, u32>(&on_check_sn) } ;

    // println!("address: {:?}",relatiave_address);


    let mut to_hook: DWORD = 0x40147D;
    
    let mut cur_protection: DWORD = 0;

    VirtualProtect(to_hook as LPVOID, 7, 0x40, &mut cur_protection);
    
    let buf =  slice::from_raw_parts_mut(to_hook as *mut u8, 7);

    for x in 0..7 {
        buf[x] = 0x90;
    }

    // let relatiave_address = std::mem::transmute::<_, u32>(&our_funct);

    // println!(" absolute: {:?} {:?}", &(our_funct as u32), relatiave_address);

    let target_address = &(our_funct as u32);

    //Calcular endereço relativo função destino - partida - 5
    let rel = target_address - to_hook - 5;

    let bytes: [u8; 4] =  transmute(rel.to_be());

    buf[0] = 0xE9;

    for x in 1..5 {
        buf[x] = bytes[4-x]
    }

    // let teste: [u8; 4] = transmute((to_hook+7).to_be());

    // println!("aq: {:?}",teste);

    // Teste jump
    // let destino: [u8; 4] = transmute(( 0x401484 as u32).to_be());
    // buf[0] = 0xE9;

    // for x in 1..5 {
    //     buf[x] = destino[4-x]
    // }

    // let bytes: [u8; 4] = [0x53,0xF8,0x10,0x10];
    // buf[1] = 0xD8;

    // let bytes = relative_addr.to_le_bytes();

    // for x in 1..5 {
    //     buf[x] = bytes[x];
    // }

    // // buf[1..5].copy_from_slice(0x401484u32.to_le_bytes());

	// // // Hook((void*)hookAddress, ourFunct, hookLength);

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
