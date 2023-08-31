mod set_distro_ascii;

use clap::Parser;
use colored::*;
use set_distro_ascii::set_distro_ascii;
use std::env;
use sysinfo::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    distro: Option<String>,
}

fn main() {
    let args = Args::parse();

    let distro = args.distro.unwrap_or(whoami::distro());

    let mut sys = System::new_all();
    sys.refresh_all();

    let shell: String = env::var("SHELL")
        .unwrap_or("Unknown".to_string())
        .to_string();

    let de: String = env::var("DESKTOP_SESSION")
        .unwrap_or("Unknown".to_string())
        .to_string();

    let distro_ascii = set_distro_ascii(distro.clone());

    let colortest1 = format!(
        "{}{}{}{}{}{}{}{}",
        "   ".on_black(),
        "   ".on_red(),
        "   ".on_yellow(),
        "   ".on_green(),
        "   ".on_cyan(),
        "   ".on_blue(),
        "   ".on_magenta(),
        "   ".on_white()
    );
    let colortest2 = format!(
        "{}{}{}{}{}{}{}{}",
        "   ".on_bright_black(),
        "   ".on_bright_red(),
        "   ".on_bright_yellow(),
        "   ".on_bright_green(),
        "   ".on_bright_cyan(),
        "   ".on_bright_blue(),
        "   ".on_bright_magenta(),
        "   ".on_bright_white()
    );

    print!("{}", distro_ascii[0]);              println!("{}@{}"            , whoami::username().bright_blue(),               whoami::hostname().bright_cyan()                          );
    print!("{}", distro_ascii[1]);              println!("{}"               , "----------------------------".white()                                                                    );
    print!("{}", distro_ascii[2]);              println!("{}: {} {}"        , "OS".bright_blue(),                             distro, whoami::arch());
    print!("{}", distro_ascii[3]);              println!("{}: {}"           , "Kernel".bright_blue(),                         sys.kernel_version().expect("Could not get Kernel")       );
    print!("{}", distro_ascii[4]);              println!("{}: {}"           , "Shell".bright_blue(),                          shell                                                     );
    print!("{}", distro_ascii[5]);              println!("{}: {}"           , "DE".bright_blue(),                             de                                                        );
    print!("{}", distro_ascii[6]);              println!("{}: {}"           , "CPU".bright_blue(),                            sys.cpus()[0].brand()                                     );
    print!("{}", distro_ascii[7]);              println!("{}: {:?}/{:?} MiB", "RAM".bright_blue(),                            sys.used_memory() / 1048576, sys.total_memory() / 1048576 );
    print!("{}", distro_ascii[8]);              println!("");
    print!("{}", distro_ascii[9]);              println!("");
    print!("{}", distro_ascii[10]);             println!("{}"               , colortest1);
    print!("{}", distro_ascii[11]);             println!("{}"               , colortest2);
    print!("{}", distro_ascii[12]);             println!("");
    print!("{}", distro_ascii[13]);             println!("");
    print!("{}", distro_ascii[14]);             println!("");
    print!("{}", distro_ascii[15]);             println!("");
    print!("{}", distro_ascii[16]);             println!("");
    print!("{}", distro_ascii[17]);             println!("");
    print!("{}", distro_ascii[18]);             println!("");
    println!("");                               println!("\x1b[;0m");
}
