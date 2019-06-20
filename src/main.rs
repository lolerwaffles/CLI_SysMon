extern crate sysinfo;
extern crate termion;

use sysinfo::{NetworkExt, ProcessorExt, System, SystemExt};
use std::{thread, time};
use termion::clear;



fn main() { 
    let mut sys = System::new();
    let ten_millis = time::Duration::from_millis(1000);

loop {
    let now = time::Instant::now();

    // Refresh Terminal:
    print!("{}[2J", 27 as char);

    // Network data:
    println!("input data : {} B", sys.get_network().get_income());
    println!("output data: {} B", sys.get_network().get_outcome());

    // CPU temperature:
    println!("{:?}", sys.get_components_list()[3]);

    // Memory information:
    println!("total memory: {} kB", sys.get_total_memory());
    println!("used memory : {} kB", sys.get_used_memory());
    println!("total swap  : {} kB", sys.get_total_swap());
    println!("used swap   : {} kB", sys.get_used_swap());
    
    // Get Proccesor Usage in %:
    for processor in sys.get_processor_list().iter().skip(1) {
        println!("Processor {} Usage: {usage:.*} %", processor.get_name(), 2, usage=processor.get_cpu_usage() * 100.0);
    }
    


    // To refresh all system information:
    sys.refresh_all();

    // Sleep to limit polling time:
    thread::sleep(ten_millis);
    assert!(now.elapsed() >= ten_millis);
    println!("{}", clear::All);
    }
}