use std::error::Error;
use std::fs;
use std::io::Write;
use std::process::Command;
use url::Url;
fn filename_from_url(url: &str) -> Option<String> {
    let url = Url::parse(url).ok()?;
    url.path_segments()?
        .last()
        .filter(|filename| !filename.is_empty())
        .map(str::to_string)
}
fn move_into_dir(src: &str, dir: &str) -> std::io::Result<()> {
    let filename = std::path::Path::new(src)
        .file_name()
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid filename"))?;

    fs::rename(src, std::path::Path::new(dir).join(filename))?;
    Ok(())
}
pub fn download(url: &str) -> Result<(), Box<dyn Error>> {
    let response = reqwest::blocking::get(url)?.error_for_status()?;
    let body = response.bytes()?;
    let filename = filename_from_url(url).ok_or("[!] Failed to extract filename from URL")?;
    let mut file = std::fs::File::create(filename)?;
    file.write_all(&body)?;
    Ok(())
}
#[derive(Debug, Clone)]
pub struct WindowsEdition {
    pub name: &'static str,
    pub xml_url: &'static str,
    pub product_key: &'static str,
}
pub static WINDOWS_EDITIONS: &[WindowsEdition] = &[
    WindowsEdition {
        name: "Education",
        product_key: "YNMGQ-8RYV3-4PGQ3-C8XTP-7CFBY",
        xml_url: "https://github.com/massgravel/hwid-kms38-tickets/releases/latest/download/Education.xml",
    },
    WindowsEdition {
        name: "Education N",
        product_key: "84NGF-MHBT6-FXBX8-QWJK7-DRR8H",
        xml_url: "https://github.com/massgravel/hwid-kms38-tickets/releases/latest/download/Education.N.xml",
    },
    WindowsEdition {
        name: "Enterprise",
        product_key: "XGVPP-NMH47-7TTHJ-W3FW7-8HV2C",
        xml_url: "https://github.com/massgravel/hwid-kms38-tickets/releases/latest/download/Enterprise.xml",
    },
    WindowsEdition {
        name: "Enterprise N",
        product_key: "3V6Q6-NQXCX-V8YXR-9QCYV-QPFCT",
        xml_url: "https://github.com/massgravel/hwid-kms38-tickets/releases/latest/download/Enterprise.N.xml",
    },
    WindowsEdition {
        name: "Enterprise LTSB 2015",
        product_key: "FWN7H-PF93Q-4GGP8-M8RF3-MDWWW",
        xml_url: "https://github.com/massgravel/hwid-kms38-tickets/releases/latest/download/Enterprise.LTSB.2015.xml",
    },
    WindowsEdition {
        name: "Enterprise LTSB 2016",
        product_key: "NK96Y-D9CD8-W44CQ-R8YTK-DYJWX",
        xml_url: "https://github.com/massgravel/hwid-kms38-tickets/releases/latest/download/Enterprise.LTSB.2016.xml",
    },
    WindowsEdition {
        name: "Enterprise LTSC 2019",
        product_key: "43TBQ-NH92J-XKTM7-KT3KK-P39PB",
        xml_url: "https://github.com/massgravel/hwid-kms38-tickets/releases/latest/download/Enterprise.LTSC.2019.xml",
    },
    WindowsEdition {
        name: "Enterprise N LTSB 2015",
        product_key: "NTX6B-BRYC2-K6786-F6MVQ-M7V2X",
        xml_url: "https://github.com/massgravel/hwid-kms38-tickets/releases/latest/download/Enterprise.N.LTSB.2015.xml",
    },
    WindowsEdition {
        name: "Enterprise N LTSB 2016",
        product_key: "2DBW3-N2PJG-MVHW3-G7TDK-9HKR4",
        xml_url: "https://github.com/massgravel/hwid-kms38-tickets/releases/latest/download/Enterprise.N.LTSB.2016.xml",
    },
    WindowsEdition {
        name: "Home",
        product_key: "YTMG3-N6DKC-DKB77-7M9GH-8HVX7",
        xml_url: "https://github.com/massgravel/hwid-kms38-tickets/releases/latest/download/Home.xml",
    },
    WindowsEdition {
        name: "Home N",
        product_key: "4CPRK-NM3K3-X6XXQ-RXX86-WXCHW",
        xml_url: "https://github.com/massgravel/hwid-kms38-tickets/releases/latest/download/Home.N.xml",
    },
    WindowsEdition {
        name: "Home China",
        product_key: "N2434-X9D7W-8PF6X-8DV9T-8TYMD",
        xml_url: "https://github.com/massgravel/hwid-kms38-tickets/releases/latest/download/Home.China.xml",
    },
    WindowsEdition {
        name: "Home Single Language",
        product_key: "BT79Q-G7N6G-PGBYW-4YWX6-6F4BT",
        xml_url: "https://github.com/massgravel/hwid-kms38-tickets/releases/latest/download/Home.Single.Language.xml",
    },
    WindowsEdition {
        name: "IoT Enterprise",
        product_key: "XQQYW-NFFMW-XJPBH-K8732-CKFFD",
        xml_url: "https://github.com/massgravel/hwid-kms38-tickets/releases/latest/download/IoT.Enterprise.xml",
    },
    WindowsEdition {
        name: "IoT Enterprise Subscription",
        product_key: "P8Q7T-WNK7X-PMFXY-VXHBG-RRK69",
        xml_url: "https://github.com/massgravel/hwid-kms38-tickets/releases/latest/download/IoT.Enterprise.Subscription.xml",
    },
    WindowsEdition {
        name: "IoT Enterprise LTSC 2021",
        product_key: "QPM6N-7J2WJ-P88HH-P3YRH-YY74H",
        xml_url: "https://github.com/massgravel/hwid-kms38-tickets/releases/latest/download/IoT.Enterprise.LTSC.2021.xml",
    },
    WindowsEdition {
        name: "IoT Enterprise LTSC 2024",
        product_key: "CGK42-GYN6Y-VD22B-BX98W-J8JXD",
        xml_url: "https://github.com/massgravel/hwid-kms38-tickets/releases/latest/download/IoT.Enterprise.LTSC.2024.xml",
    },
    WindowsEdition {
        name: "IoT Enterprise LTSC Subscription 2024",
        product_key: "N979K-XWD77-YW3GB-HBGH6-D32MH",
        xml_url: "https://github.com/massgravel/hwid-kms38-tickets/releases/latest/download/IoT.Enterprise.LTSC.Subscription.2024.xml",
    },
    WindowsEdition {
        name: "Pro",
        product_key: "VK7JG-NPHTM-C97JM-9MPGT-3V66T",
        xml_url: "https://github.com/massgravel/hwid-kms38-tickets/releases/latest/download/Pro.xml",
    },
    WindowsEdition {
        name: "Pro N",
        product_key: "2B87N-8KFHP-DKV6R-Y2C8J-PKCKT",
        xml_url: "https://github.com/massgravel/hwid-kms38-tickets/releases/latest/download/Pro.N.xml",
    },
    WindowsEdition {
        name: "Pro Education",
        product_key: "8PTT6-RNW4C-6V7J2-C2D3X-MHBPB",
        xml_url: "https://github.com/massgravel/hwid-kms38-tickets/releases/latest/download/Pro.Education.xml",
    },
    WindowsEdition {
        name: "Pro Education N",
        product_key: "GJTYN-HDMQY-FRR76-HVGC7-QPF8P",
        xml_url: "https://github.com/massgravel/hwid-kms38-tickets/releases/latest/download/Pro.Education.N.xml",
    },
    WindowsEdition {
        name: "Pro for Workstations",
        product_key: "DXG7C-N36C4-C4HTG-X4T3X-2YV77",
        xml_url: "https://github.com/massgravel/hwid-kms38-tickets/releases/latest/download/Pro.for.Workstations.xml",
    },
    WindowsEdition {
        name: "Pro N for Workstations",
        product_key: "WYPNQ-8C467-V2W6J-TX4WX-WT2RQ",
        xml_url: "https://github.com/massgravel/hwid-kms38-tickets/releases/latest/download/Pro.N.for.Workstations.xml",
    },
    WindowsEdition {
        name: "S",
        product_key: "V3WVW-N2PV2-CGWC3-34QGF-VMJ2C",
        xml_url: "https://github.com/massgravel/hwid-kms38-tickets/releases/latest/download/Cloud.S.xml",
    },
    WindowsEdition {
        name: "S N",
        product_key: "NH9J3-68WK7-6FB93-4K3DF-DJ4F6",
        xml_url: "https://github.com/massgravel/hwid-kms38-tickets/releases/latest/download/Cloud.S.N.xml",
    },
    WindowsEdition {
        name: "SE",
        product_key: "KY7PN-VR6RX-83W6Y-6DDYQ-T6R4W",
        xml_url: "https://github.com/massgravel/hwid-kms38-tickets/releases/latest/download/CloudEdition.SE.xml",
    },
    WindowsEdition {
        name: "SE N",
        product_key: "K9VKN-3BGWV-Y624W-MCRMQ-BHDCD",
        xml_url: "https://github.com/massgravel/hwid-kms38-tickets/releases/latest/download/CloudEdition.SE.N.xml",
    },
    WindowsEdition {
        name: "Team",
        product_key: "XKCNC-J26Q9-KFHD2-FKTHY-KD72Y",
        xml_url: "https://github.com/massgravel/hwid-kms38-tickets/releases/latest/download/Team.xml",
    },
];

pub fn find_windows_edition(edition: &str) -> Option<&'static WindowsEdition> {
    WINDOWS_EDITIONS.iter().find(|entry| entry.name == edition)
}

pub fn save_ticket_by_edition(edition: &str) {
    let mut url = String::new();
    for i in WINDOWS_EDITIONS {
        if i.name == edition {
            url = i.xml_url.to_string();
            println!("[*] Downloading tickets for {}", i.name);
        }
    }
    if url.is_empty() {
        eprintln!("[!] Unsupported edition!");
        return;
    }
    let ticket_filename = filename_from_url(&url).expect("[!] Failed to extract filename from URL");
    download(&url).expect("[!] Failed to download ticket");
    move_into_dir(
        &ticket_filename,
        "C:\\ProgramData\\Microsoft\\Windows\\ClipSVC\\GenuineTicket",
    )
    .expect("[!] Failed to move the file! Re-run with admin!");
}

pub fn apply_product_key(edition: &str) -> std::process::Output {
    let mut product_key = String::new();
    for i in WINDOWS_EDITIONS {
        if i.name == edition {
            product_key = i.product_key.to_string();
        }
    }
    println!("[*] Setting {} product key to {}", edition, product_key);
    let output = Command::new("cmd")
        .args([
            "/C",
            &format!("slmgr.vbs /upk && slmgr.vbs /ipk {}", product_key),
        ]) // Use ["/C", "..."] on Windows to run a command and terminate
        .output()
        .expect("[!] Failed to execute command");
    return output;
}
pub fn activate() -> std::process::Output {
    println!("[*] Activating Windows...");
    let output = Command::new("cmd")
        .args(["/C", "slmgr.vbs /ato"])
        .output()
        .expect("[!] Failed to execute command");
    return output;
}
pub fn check_activation() -> std::process::Output {
    println!("[*] Making sure everything is done correctly....");
    let output = Command::new("cmd")
        .args(["/C", "slmgr.vbs /dlv"])
        .output()
        .expect("[!] Failed to execute command");
    return output;
}
