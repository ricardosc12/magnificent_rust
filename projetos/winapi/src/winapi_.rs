use winapi::shared::minwindef::{ DWORD, FALSE};
use winapi::um::handleapi::{INVALID_HANDLE_VALUE, CloseHandle};
use winapi::um::winnt::{HANDLE, PROCESS_ALL_ACCESS};
use std::mem::{MaybeUninit, size_of};
use winapi::um::tlhelp32::{
    CreateToolhelp32Snapshot, TH32CS_SNAPPROCESS, 
    PROCESSENTRY32, Process32First, Process32Next, 
    TH32CS_SNAPMODULE, TH32CS_SNAPMODULE32, MODULEENTRY32, 
    Module32First, Module32Next
};
use winapi::um::processthreadsapi::{OpenProcess};
use std::{ptr};
use winapi::vc::vadefs::uintptr_t;
use winapi::um::memoryapi::ReadProcessMemory;
use winapi::shared::basetsd::SIZE_T;
use std::{thread, time::Duration};

use crate::auxiliares::{cmp_array_string};


pub unsafe fn get_proc_id(proc_name: &str) ->  Vec<u32> {

    let mut proc_id: Vec<u32> = Vec::new();

    let h_snap: HANDLE =  CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);

    if h_snap != INVALID_HANDLE_VALUE {

        let mut proc_entry = MaybeUninit::<PROCESSENTRY32>::uninit();

        proc_entry.assume_init_mut().dwSize =  size_of::<PROCESSENTRY32>() as u32;

        if Process32First(h_snap, proc_entry.as_mut_ptr()) != 0 {
            loop {
                let name_source = (*proc_entry.as_ptr()).szExeFile;
                let test = cmp_array_string(proc_name,&name_source);
                if test == 1 {
                    proc_id.push((*proc_entry.as_ptr()).th32ProcessID);
                    Process32Next(h_snap, proc_entry.as_mut_ptr());
                    continue;
                }
                else if test == 2 {
                    break;
                }
                else {
                    Process32Next(h_snap, proc_entry.as_mut_ptr());
                }
                
                // thread::sleep(Duration::from_millis(25));
            }
        }
    }

    CloseHandle(h_snap);

    proc_id
    
}

pub unsafe fn get_module_base_addr(proc_id: DWORD, mod_name: &str) -> uintptr_t {
    let mut mod_base_addr: uintptr_t = 0;
    let h_snap: HANDLE = CreateToolhelp32Snapshot(TH32CS_SNAPMODULE | TH32CS_SNAPMODULE32, proc_id);
    
    if h_snap != INVALID_HANDLE_VALUE {

        // let mut mod_entry = MODULEENTRY32::default(); Deve haver o default implementado ?

        let mut mod_entry = MaybeUninit::<MODULEENTRY32>::uninit();

        mod_entry.assume_init_mut().dwSize = size_of::<MODULEENTRY32>() as u32;

        if Module32First(h_snap, mod_entry.as_mut_ptr()) != 0 {
            
            mod_base_addr = loop {
                if (*mod_entry.as_ptr()).th32ProcessID == proc_id {
                    break (*mod_entry.as_ptr()).modBaseAddr as uintptr_t;
                }
                // let name_source = (*mod_entry.as_ptr()).szModule;
                // if cmp_array_string(mod_name,&name_source) == 1{
                //     println!("ID: {:?}",(*mod_entry.as_ptr()).th32ProcessID);
                //     break (*mod_entry.as_ptr()).modBaseAddr as uintptr_t;
                // }
                Module32Next(h_snap,  mod_entry.as_mut_ptr());
            }

        }

    }
    CloseHandle(h_snap);
    mod_base_addr
}

pub unsafe fn find_dma_addy(h_proc: HANDLE, ptr: uintptr_t, offsets: &mut [uintptr_t]) ->  uintptr_t {
    
    let mut addr: uintptr_t = ptr;

    for off in offsets {
        ReadProcessMemory(h_proc, addr as *const _, &mut addr as *mut _ as *mut _, size_of::<u32>() as SIZE_T, ptr::null_mut());
        addr += *off;
    }
    addr
}

pub unsafe fn read_memory(h_proc: HANDLE, ptr: uintptr_t) -> uintptr_t {
    let mut valor: uintptr_t = 0;

    ReadProcessMemory(h_proc, ptr as *const _, &mut valor as *mut _ as *mut _, size_of::<u32>() as SIZE_T, ptr::null_mut());

    valor
}

pub unsafe fn open_process(proc_id: u32) -> HANDLE {
    OpenProcess(PROCESS_ALL_ACCESS, FALSE, proc_id)
}