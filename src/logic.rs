#![cfg(windows)]
pub mod detector;
pub mod hwid;
pub mod ohook;
use crate::logic::detector::{detect_office_products, windows_edition};
use crate::logic::hwid::{
    activate, apply_product_key, check_activation, find_windows_edition, save_ticket_by_edition,
};
use crate::logic::ohook::{find_office_product, install_spcc};
pub fn activate_windows() {
    println!("[*] Getting necesarry info");
    let edition = windows_edition().expect("[!] Failed to get windows edition!");
    let Some(edition_info) = find_windows_edition(&edition) else {
        eprintln!("[!] The {} edition is not supported!", edition);
        return;
    };
    println!(
        "[*] Detected supported Windows edition: {}",
        edition_info.name
    );
    save_ticket_by_edition(&edition);
    let output = apply_product_key(&edition);
    println!("{:?}", output);
    println!("{:?}", activate());
    println!("{:?}", check_activation());
}
pub fn activate_office() {
    for i in detect_office_products().expect("[!] Failed to find MS Office") {
        let Some(key) = find_office_product(i.as_str()) else {
            eprintln!("[!] The {} is not supported!", i);
            continue;
        };
        println!("[*] Detected supported Office product: {}", i);
        if install_spcc(key).status.success() {
            println!("[*] Succesfully activated {}", i);
        } else {
            eprintln!("[!] Failed to activate {}", i);
        }
    }
}
