use std::env;
use sysinfo::{
    ProcessExt, 
    System, 
    SystemExt
};

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
            }
        }

        if !found_process {
            println!("The process [{}] was not found.", process_name);
        }

    }

}
