pub mod logic;
use crate::logic::{activate_office, activate_windows};
use std::io;
use std::io::Write;
fn more() {
    println!(
        "This program lets you activate Windows 10/11 and MS Office 2016-365 for free using HWID/Ohook methods."
    );
    println!(
        "This program is completely free and open source and is licensed via GNU GPL v3 license."
    );
    println!(
        "Project links:\nGitHub: https://github.com/mrrabyss/MSActiverse\nDev website: https://mrrabyss.github.io\nContact & donate: https://guns.lol/mrrabyss"
    );
}

fn main() {
    const BANNER: &'static str = r#"
 ██████   ██████  █████████    █████████             █████     ███                                                            ■ ■                  ■ ■                                                                                  
░░██████ ██████  ███░░░░░███  ███░░░░░███           ░░███     ░░░                                                              \ \   /\_/\_/\_/\  / /
 ░███░█████░███ ░███    ░░░  ░███    ░███   ██████  ███████   ████  █████ █████  ██████  ████████   █████   ██████               \ (='_`  _  `_`=) /
 ░███░░███ ░███ ░░█████████  ░███████████  ███░░███░░░███░   ░░███ ░░███ ░░███  ███░░███░░███░░███ ███░░   ███░░███               `-\___________/-' 
 ░███ ░░░  ░███  ░░░░░░░░███ ░███░░░░░███ ░███ ░░░   ░███     ░███  ░███  ░███ ░███████  ░███ ░░░ ░░█████ ░███████                    /_/   \_\
 ░███      ░███  ███    ░███ ░███    ░███ ░███  ███  ░███ ███ ░███  ░░███ ███  ░███░░░   ░███      ░░░░███░███░░░  
 █████     █████░░█████████  █████   █████░░██████   ░░█████  █████  ░░█████   ░░██████  █████     ██████ ░░██████ 
░░░░░     ░░░░░  ░░░░░░░░░  ░░░░░   ░░░░░  ░░░░░░     ░░░░░  ░░░░░    ░░░░░     ░░░░░░  ░░░░░     ░░░░░░   ░░░░░░  

Made by mrrabyss.

╔════════════════════════════════════════════════════════════════════════════════════════════╗
║ 1. Activate Windows(Windows 10/11, HWID)                2.Activate Office(2016-365, Ohook) ║
║ 3. Learn more                                           4. Exit                            ║
╚════════════════════════════════════════════════════════════════════════════════════════════╝
    "#;
    loop {
        print!("\x1B[2J\x1B[1;1H");
        println!("{}", BANNER);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("[!] Failed to read line");
        match input.trim().parse::<u8>().expect("Please enter a valid option!") {
            1 => activate_windows(),
            2 => activate_office(),
            3 => more(),
            4 => std::process::exit(0),
            _ => eprintln!("Please enter a valid option!"),
        }
        print!("Press ENTER to continue...");
        io::stdout().flush().unwrap();
        let mut _input = String::new();
        io::stdin().read_line(&mut _input).unwrap();
    }
}
