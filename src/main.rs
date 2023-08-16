mod set_distro_ascii;

use std::env;
use colored::*;
use sysinfo::*;
use set_distro_ascii::set_distro_ascii;


fn main() {

    let args: Vec<String> = env::args().collect();

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

    let os_name: String;
    if args.len() >= 2 as usize {
        os_name = args[1].to_string();
    } else {
        os_name = osinfo.os_type().to_string();
    }
    let distro_ascii = set_distro_ascii(os_name.clone());

    print!("{}", distro_ascii[0]);
    println!("{}@{}", user.bright_blue(),               sys.host_name().expect("Could not get hostname").bright_cyan());
    print!("{}", distro_ascii[1]);
    println!("----------------------------");
    print!("{}", distro_ascii[2]);
    println!("{}: {} {}", "OS".bright_blue(),           os_name, osinfo.architecture().unwrap());
    print!("{}", distro_ascii[3]);
    println!("{}: {}", "Kernel".bright_blue(),          sys.kernel_version().expect("Could not get Kernel"));
    print!("{}", distro_ascii[4]);
    println!("{}: {}", "Shell".bright_blue(),           shell);
    print!("{}", distro_ascii[5]);
    println!("{}: {}", "DE".bright_blue(),              de);
    print!("{}", distro_ascii[6]);
    println!("{}: {}", "CPU".bright_blue(),             sys.cpus()[0].brand());
    print!("{}", distro_ascii[7]);
    println!("{}: {:?}/{:?} MiB", "RAM".bright_blue(),  sys.used_memory() / 1048576, sys.total_memory() / 1048576);
    print!("{}", distro_ascii[8]);
    println!("");
    print!("{}", distro_ascii[9]);
    println!("");
    print!("{}", distro_ascii[10]);
    println!("");
    print!("{}", distro_ascii[11]);
    println!("");
    print!("{}", distro_ascii[12]);
    println!("");
    print!("{}", distro_ascii[13]);
    println!("");
    print!("{}", distro_ascii[14]);
    println!("");
    print!("{}", distro_ascii[15]);
    println!("");
    print!("{}", distro_ascii[16]);
    println!("");
    print!("{}", distro_ascii[17]);
    println!("");
    print!("{}", distro_ascii[18]);
    println!("");

}
