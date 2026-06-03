#![cfg(windows)]

use winreg::RegKey;
use winreg::enums::HKEY_LOCAL_MACHINE;

pub fn windows_edition() -> Result<String, Box<dyn std::error::Error>> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let cv = hklm.open_subkey(r"SOFTWARE\Microsoft\Windows NT\CurrentVersion")?;
    let product_name: String = cv.get_value("ProductName")?;

    let edition = product_name
        .splitn(3, ' ')
        .nth(2)
        .unwrap_or(&product_name)
        .to_string();

    Ok(edition)
}
pub fn detect_office_products() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let mut products: Vec<String> = Vec::new();

    let search_paths = [
        r"SOFTWARE\Microsoft\Office",
        r"SOFTWARE\WOW6432Node\Microsoft\Office",
    ];

    for base_path in &search_paths {
        let Ok(office_key) = hklm.open_subkey(base_path) else { continue };

        for ver_name in office_key.enum_keys().filter_map(Result::ok) {
            if !ver_name.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false) {
                continue;
            }

            let Ok(ver_key) = office_key.open_subkey(&ver_name) else { continue };

            if let Ok(c2r) = ver_key.open_subkey(r"ClickToRun\Configuration") {
                if let Ok(ids) = c2r.get_value::<String, _>("ProductReleaseIds") {
                    for id in ids.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()) {
                        if !products.contains(&id) {
                            products.push(id);
                        }
                    }
                }
            }
        }
    }

    Ok(products)
}