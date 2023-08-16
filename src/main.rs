use std::env;
use colored::*;
use sysinfo::*;


fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();

    let osinfo = os_info::get();

    let user: String = env::var("USER") 
        .expect("Could not get user")
        .to_string();

    let shell: String = env::var("SHELL") 
        .expect("Could not get shell")
        .to_string();

    let de: String = env::var("DESKTOP_SESSION") 
        .expect("Could not get de")
        .to_string();

    println!("{}@{}", user.bright_blue(),               sys.host_name().expect("Could not get hostname").bright_cyan());
    
    println!("----------------------------");
    
    println!("{}: {} {}", "OS".bright_blue(),           osinfo.os_type(), osinfo.architecture().unwrap());
    
    println!("{}: {}", "Kernel".bright_blue(),          sys.kernel_version().expect("Could not get Kernel"));
    
    println!("{}: {}", "Shell".bright_blue(),           shell);
    
    println!("{}: {}", "DE".bright_blue(),              de);
    
    println!("{}: {}", "CPU".bright_blue(),             sys.cpus()[0].brand());
    
    println!("{}: {:?}/{:?} MiB", "RAM".bright_blue(),  sys.used_memory() / 1048576, sys.total_memory() / 1048576);
}
