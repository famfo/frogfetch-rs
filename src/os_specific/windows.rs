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

    // Get the boot time using wmic path Win32_OperatingSystem get LastBootUpTime
    let boot_time = String::from_utf8(
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

    let current_time = &String::from_utf8(
        Command::new("powershell.exe")
        .arg("date")
        .output()
        .expect("Failed to execute powershell date")
        .stdout,
    ).unwrap_or_default()[10..];
        
    let current_time = current_time.split_whitespace().collect::<Vec<&str>>();
    let current_day = &current_time[0][..1];
    let current_month = &current_time[1];
    let current_month = match current_month {
        &"January" => 1,
        &"Feburary" => 2,
        &"March" => 3,
        &"April" => 4,
        &"May" => 5,
        &"June" => 6,
        &"July" => 7,
        &"August" => 8,
        &"September" => 9,
        &"October" => 10,
        &"November" => 11,
        &"December" => 12,
        _ => 0,
    };

    println!("Current time: {} at {}-{}-{}", current_time[3], current_day, current_month, 0);

    let boot_year = &boot_time[..4];
    let boot_month = &boot_time[4..6];
    let boot_day = &boot_time[6..8];
    let boot_hour: u32 =std::str::FromStr::from_str(&boot_time[8..10]).unwrap();
    let boot_minute = &boot_time[10..12];
    let boot_second = &boot_time[12..14];
    let time_zone = &boot_time[21..25];

    let uptime = format!("Boot time: {}:{}:{} at {}-{}-{} (GMT{})", boot_hour, boot_minute, boot_second, boot_day, boot_month, boot_year, time_zone);

    // Lie to the Windows users about the shell
    let shell = "Powershell".to_string();

    // Lie to the Windows users about the terminal (since this is the only one it works corretctly in anyways)
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
