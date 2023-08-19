pub fn set_distro_ascii(os_name: String) -> Vec<&'static str> {
    let distro_ascii: Vec<&str>;

    match os_name.as_str() {
        "Arch Linux" => distro_ascii = arch_linux(),
        "EndeavourOS" => distro_ascii = endeavour_os(),
        _ => distro_ascii = tux(),
    }
    
    if os_name == "Arch Linux".to_string() {
             
    }
    return distro_ascii;
}

fn arch_linux() -> Vec<&'static str> {
    let mut ascii = vec!["";19];
    ascii[0]  = "\x1b[38;2;23;147;209m                   ▄                     ";
    ascii[1]  = "\x1b[38;2;23;147;209m                  ▟█▙                    ";
    ascii[2]  = "\x1b[38;2;23;147;209m                 ▟███▙                   ";
    ascii[3]  = "\x1b[38;2;23;147;209m                ▟█████▙                  ";
    ascii[4]  = "\x1b[38;2;23;147;209m               ▟███████▙                 ";
    ascii[5]  = "\x1b[38;2;23;147;209m              ▂▔▀▜██████▙                ";
    ascii[6]  = "\x1b[38;2;23;147;209m             ▟██▅▂▝▜█████▙               ";
    ascii[7]  = "\x1b[38;2;23;147;209m            ▟█████████████▙              ";
    ascii[8]  = "\x1b[38;2;23;147;209m           ▟███████████████▙             ";
    ascii[9]  = "\x1b[38;2;23;147;209m          ▟█████████████████▙            ";
    ascii[10] = "\x1b[38;2;23;147;209m         ▟███████████████████▙           ";
    ascii[11] = "\x1b[38;2;23;147;209m        ▟█████████▛▀▀▜████████▙          ";
    ascii[12] = "\x1b[38;2;23;147;209m       ▟████████▛      ▜███████▙         ";
    ascii[13] = "\x1b[38;2;23;147;209m      ▟█████████        ████████▙        ";
    ascii[14] = "\x1b[38;2;23;147;209m     ▟██████████        █████▆▅▄▃▂       ";
    ascii[15] = "\x1b[38;2;23;147;209m    ▟██████████▛        ▜█████████▙      ";
    ascii[16] = "\x1b[38;2;23;147;209m   ▟██████▀▀▀              ▀▀██████▙     ";
    ascii[17] = "\x1b[38;2;23;147;209m  ▟███▀▘                       ▝▀███▙    ";
    ascii[18] = "\x1b[38;2;23;147;209m ▟▛▀                               ▀▜▙   ";
    return ascii;
}

fn endeavour_os() -> Vec<&'static str> {
    let mut ascii = vec!["";19];
    ascii[0]  = "                     ./o.                  ";
    ascii[1]  = "                   ./sssso-                ";
    ascii[2]  = "                 `:osssssss+-              ";
    ascii[3]  = "               `:+sssssssssso/.            ";
    ascii[4]  = "             `-/ossssssssssssso/.          ";
    ascii[5]  = "           `-/+sssssssssssssssso+:`        ";
    ascii[6]  = "         `-:/+sssssssssssssssssso+/.       ";
    ascii[7]  = "       `.://osssssssssssssssssssso++-      ";
    ascii[8]  = "      .://+ssssssssssssssssssssssso++:     ";
    ascii[9]  = "    .:///ossssssssssssssssssssssssso++:    ";
    ascii[10] = "  `:////ssssssssssssssssssssssssssso+++.   ";
    ascii[11] = "`-////+ssssssssssssssssssssssssssso++++-   ";
    ascii[12] = " `..-+oosssssssssssssssssssssssso+++++/`   ";
    ascii[13] = "   ./++++++++++++++++++++++++++++++/:.     ";
    ascii[14] = "   `:::::::::::::::::::::::::------``      ";
    ascii[15] = "";
    ascii[16] = "";
    ascii[17] = "";
    ascii[18] = "";
    return ascii;
}

fn tux() -> Vec<&'static str> {
    let mut ascii = vec!["";19];
    ascii[0]  = "\x1b[;90m        #####                     ";
    ascii[1]  = "\x1b[;90m       #######                    ";
    ascii[2]  = "\x1b[;90m       ##\x1b[;97mO\x1b[;90m#\x1b[;97mO\x1b[;90m##                    ";
    ascii[3]  = "\x1b[;90m       #\x1b[;93m#####\x1b[;90m#                    ";
    ascii[4]  = "\x1b[;90m     ##\x1b[;97m##\x1b[;93m###\x1b[;97m##\x1b[;90m##                  ";
    ascii[5]  = "\x1b[;90m    #\x1b[;97m##########\x1b[;90m##                 ";
    ascii[6]  = "\x1b[;90m   #\x1b[;97m############\x1b[;90m##                ";
    ascii[7]  = "\x1b[;90m   #\x1b[;97m############\x1b[;90m###               ";
    ascii[8]  = "\x1b[;93m  ##\x1b[;90m#\x1b[;97m###########\x1b[;90m##\x1b[;93m#       ";
    ascii[9]  = "\x1b[;93m######\x1b[;90m#\x1b[;97m#######\x1b[;90m#\x1b[;93m######     ";
    ascii[10] = "\x1b[;93m#######\x1b[;90m#\x1b[;97m#####\x1b[;90m#\x1b[;93m#######     ";
    ascii[11] = "\x1b[;93m  #####\x1b[;90m#######\x1b[;93m#####                 ";
    ascii[12] = "";
    ascii[13] = "";
    ascii[14] = "";
    ascii[15] = "";
    ascii[16] = "";
    ascii[17] = "";
    ascii[18] = "";
    return ascii;
}
