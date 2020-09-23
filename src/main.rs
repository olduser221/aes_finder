use std::env;
// use process_memory_reader::Process;
use sysinfo::{
    ProcessExt, 
    System, 
    SystemExt
};

// fn read_memory(process_id: u32, process_name: &str) {
//     let process = Process::open_process(process_id).unwrap();
//     let base_address = process.base_address(process_name).unwrap();
//     process.read_u8(base_address).unwrap();
//     println!("{:?}", process.read_u8(base_address + 0x127).unwrap());
// }

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {

        let ref process_name = &args[1];

        let mut sys = System::new_all();
    
        sys.refresh_all();
    
        let mut found_process: bool = false;
        for (pid, process) in sys.get_processes() {
            if &process.name() == process_name {
                if !found_process {
                    println!("Found: {}", process.name());
                    found_process = true;
                }
                println!("- PID: {}", pid);
                // read_memory(*pid as u32, process.name());
            }
        }

        if !found_process {
            println!("The process [{}] was not found.", process_name);
        }

    }

}
