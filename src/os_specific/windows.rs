use std::process::Command;

pub fn get_info() {
    // Get the current user and hostname using whoami
    let hostname_user = String::from_utf8(
        Command::new("whoami")
            .output()
            .expect("Failed to execute whoami")
            .stdout,
    )
    .unwrap_or_default()
    .trim()
    .to_string();

    let (hostname, user) = hostname_user.split_once("\\").unwrap_or_default();

    // Get the os using wmic os get Caption
    let os = String::from_utf8(
        Command::new("wmic")
            .arg("os")
            .arg("get")
            .arg("Caption")
            .output()
            .expect("Failed to execute wmic")
            .stdout,
    )
    .unwrap_or_default()[8..]
        .trim()
        .to_string();

    // Get the architecture using wmic os get OSArchitecture
    let architecture = String::from_utf8(
        Command::new("wmic")
            .arg("os")
            .arg("get")
            .arg("OSArchitecture")
            .output()
            .expect("Failed to execute wmic")
            .stdout,
    )
    .unwrap_or_default()[14..]
        .trim()
        .to_string();

    // Get the kernel name using wmic os get Version
    let kernel = String::from_utf8(
        Command::new("wmic")
            .arg("os")
            .arg("get")
            .arg("Version")
            .output()
            .expect("Failed to execute wmic")
            .stdout,
    )
    .unwrap_or_default()[8..]
        .trim()
        .to_string();

    // Get the uptime using wmic path Win32_OperatingSystem get LastBootUpTime
    let uptime = String::from_utf8(
        Command::new("wmic")
            .arg("path")
            .arg("Win32_OperatingSystem")
            .arg("get")
            .arg("LastBootUpTime")
            .output()
            .expect("Failed to execute wmic")
            .stdout,
    )
    .unwrap_or_default()[14..]
        .trim()
        .to_string();

    let boot_year = &uptime[..4];
    let boot_month = &uptime[4..6];
    let boot_day = &uptime[6..8];
    let mut boot_hour: u32 =std::str::FromStr::from_str(&uptime[8..10]).unwrap();
    let boot_minute = &uptime[10..12];
    let boot_second = &uptime[12..14];
    let time_zone = &uptime[21..25];

    if time_zone.chars().nth(0) == Some('+') {
        let offset: u32 = std::str::FromStr::from_str(&time_zone[1..]).unwrap();
        boot_hour = boot_hour - (offset / 60);
    } else {
        let offset: u32 = std::str::FromStr::from_str(&time_zone[1..]).unwrap();
        boot_hour = boot_hour + (offset / 60);
    }

    let uptime = format!("Boot time: {}:{}:{} at {}-{}-{} (GMT{})", boot_hour, boot_minute, boot_second, boot_day, boot_month, boot_year, time_zone);

    // Get the default shell using the $SHELL enviromental variable
    let shell = "Powershell".to_string();

    // Get the default terminal using the $TERM enviromental variable
    let term = "Windows Terminal".to_string();

    // Get the CPU name and manufacturer using wmic cpu get name
    let cpu = String::from_utf8(
        std::process::Command::new("wmic")
            .arg("cpu")
            .arg("get")
            .arg("name")
            .output()
            .expect("Failed to execute wmic")
            .stdout,
    )
    .unwrap_or_default()[4..]
        .trim()
        .to_string();

    let cpu = cpu.split("\r\n").collect::<Vec<&str>>()[0].to_string();

    // Get the total memory using wmic memorychip get capacity
    let memory: u64 = std::str::FromStr::from_str(
        String::from_utf8(
            Command::new("wmic")
                .arg("memorychip")
                .arg("get")
                .arg("capacity")
                .output()
                .expect("Failed to execute wmic")
                .stdout,
        )
        .unwrap_or_default()[8..]
            .trim(),
    )
    .unwrap();

    let mut memory = (memory / 1024).to_string();
    memory.push_str(" kB");

    // Get the system language using wmic os get MUILanguages
    let lang = "TODO".to_string();

    crate::print_frog(
        user.to_string(),
        hostname.to_string(),
        os,
        architecture,
        kernel,
        uptime,
        shell,
        term,
        cpu,
        memory,
        lang,
    );
}
