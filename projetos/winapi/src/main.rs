use std::sync::{Arc, Mutex};
use std::{thread::{self, JoinHandle}, time::Duration};
use winapi::um::winnt::{HANDLE};
use winapi::vc::vadefs::uintptr_t;
mod auxiliares;
mod winapi_;

use crate::winapi_::{get_proc_id, get_module_base_addr, find_dma_addy, read_memory, open_process};

#[derive(Debug)]
struct Processo {
    id:u32,
    base:u32,
    handle: HANDLE,
    valor_addr: u32,
}

impl Processo {
    unsafe fn get_valor(&mut self) -> uintptr_t {
        
        // let mut offsets: [uintptr_t; 7] = [0x20, 0x4, 0x4, 0x1C8, 0x2C, 0x1A4, 0x54C];

        if self.valor_addr != 0 {
            return read_memory(self.handle, self.valor_addr as uintptr_t) as uintptr_t;
        }

        let mut offsets: [uintptr_t; 7] = [0x10, 0xD0, 0x668, 0x14, 0x50, 0x1A4, 0x5AC];

        let dynamic_address: uintptr_t = self.base as uintptr_t + 0x00007000;

        self.valor_addr = find_dma_addy(self.handle, dynamic_address, &mut offsets) as u32;

        read_memory(self.handle, self.valor_addr as uintptr_t) as uintptr_t
    }
}

unsafe impl Send for Processo {}

impl Default for Processo {
    fn default() -> Self {
        Self { id: 0, base: 0, handle: 0 as HANDLE, valor_addr: 0 }
    }
}


unsafe fn get_new_process(processos: Arc<Mutex<Vec<Processo>>>,nome: &str) {
    loop {
        let ids_process = get_proc_id(nome);
        if processos.lock().unwrap().len() == 0 {
            for processo in ids_process {
                processos.lock().unwrap().push(Processo {
                    id:processo,
                    base: get_module_base_addr(processo, nome) as u32,
                    handle: open_process(processo),
                    ..Default::default()
                });
            };
        } else {
            // let css = processos.lock().unwrap().iter();
            let mut to_remove: Vec<usize> = Vec::new();
            let mut to_add: Vec<u32> = Vec::new();

            for novo in &ids_process {
                let mut achou = false;
                for prcs in processos.lock().unwrap().iter() {
                    if *novo == prcs.id {
                        achou = true;
                        break;
                    }
                }
                if achou == false {
                    to_add.push(*novo);
                }
            }
            for (index, prcs) in processos.lock().unwrap().iter().enumerate() {
                if !ids_process.contains(&(*prcs).id) {
                    to_remove.push(index);
                }
            }
            for del in to_remove {
                processos.lock().unwrap().swap_remove(del);
            }
            for add in to_add {
                processos.lock().unwrap().push( Processo { 
                    id: add, 
                    base: get_module_base_addr(add, nome) as u32,
                    handle: open_process(add),
                    ..Default::default()
                });
            }
        }
        
        // for ps in get_proc_id("a.exe") {
        //     if prss.len() > 0 {

        //     }
        // }
        // println!("Processos: {:?}",prss);
        thread::sleep(Duration::from_millis(1000));
    }
}

fn main() {
    unsafe {

        let vetor: Arc<Mutex<Vec<Processo>>> = Arc::new(Mutex::new(Vec::new()));
        let mut handles: Vec<JoinHandle<()>> = vec![];

        {
            let mut vetor = Arc::clone(&vetor);
            let handle = thread::spawn(move || get_new_process(vetor,"a.exe"));
            handles.push(handle);
            
            
        }
        {
            let vetor = Arc::clone(&vetor);
            let handle = thread::spawn(move || {
                loop {
                    println!("Pro: {:?}",vetor.lock().unwrap());
                    // println!("as");
                    if vetor.lock().unwrap().len() > 0 {
                        println!("Valor: {:?}",vetor.lock().unwrap()[0].get_valor());
                    }
                    thread::sleep(Duration::from_millis(1000));
                }
            });
            handles.push(handle);
        }

        // let mut vetor_process: Vec<u32> = Vec::new();

        // vetor_process = get_proc_id("a.exe");

        // if vetor_process.len() == 0{
        //     println!("process not found !");
        //     return;
        // }

        // println!("Qtd: {:?}",vetor_process.len());

        // println!("Processos: {:?}",vetor_process);

        // let module_base_addr: uintptr_t = get_module_base_addr(proc_id, "a.exe");

        // let h_process: HANDLE = OpenProcess(PROCESS_ALL_ACCESS, FALSE, proc_id);

        // // TerminateProcess(h_process,0);

        // let mut offsets: [uintptr_t; 7] = [0x20, 0x4, 0x4, 0x1C8, 0x2C, 0x1A4, 0x54C];

        // let dynamic_address: uintptr_t = module_base_addr + 0x00007000;

        // let variable_addr: uintptr_t = find_dma_addy(h_process, dynamic_address, &mut offsets);

        // let mut valor: uintptr_t = 0;

        
        
        // println!("PROCESS ID {:?}",proc_id);

        // println!("MODULE BASE ADDR {:?}",module_base_addr);

        // println!("H_PROCESS {:?}",h_process);

        // loop {
        //     valor = read_memory(h_process,variable_addr);
        //     println!("Variável: {:?}",valor);
        //     thread::sleep(Duration::from_millis(1000));
        // }

        for handle in handles {
            handle.join().unwrap();
        }
    }
    // pause();
}
// 140695471587328