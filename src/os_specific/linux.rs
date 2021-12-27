use std::process::Command;

pub fn get_info() {
    // Get the current user using the $USER enviromental variable
    let user = std::env::var("USER").unwrap_or_default();

    // Get the hostname from /etc/hostname
    let hostname = String::from_utf8(
        Command::new("cat")
            .arg("/etc/hostname")
            .output()
            .expect("Failed to execute cat")
            .stdout,
    )
    .unwrap_or_default()
    .trim()
    .to_string();

    // Get the os using uname -s
    let os = String::from_utf8(
        Command::new("uname")
            .arg("-o")
            .output()
            .expect("Failed to execute uname")
            .stdout,
    )
    .unwrap_or_default()
    .trim()
    .to_string();

    // Get the architecture using uname -m
    let architecture = String::from_utf8(
        Command::new("uname")
            .arg("-m")
            .output()
            .expect("Failed to execute uname")
            .stdout,
    )
    .unwrap_or_default()
    .trim()
    .to_string();

    // Get the kernel name using uname -r
    let kernel = String::from_utf8(
        Command::new("uname")
            .arg("-r")
            .output()
            .expect("Failed to execute uname")
            .stdout,
    )
    .unwrap_or_default()
    .trim()
    .to_string();

    // Get the uptime using uptime
    // Trim out the first three character
    let uptime = String::from_utf8(
        Command::new("uptime")
            .arg("-p")
            .output()
            .expect("Failed to execute uptime")
            .stdout,
    )
    .unwrap_or_default()
    .trim()[3..]
        .to_string();

    // Get the default shell using the $SHELL enviromental variable
    let shell = std::env::var("SHELL").unwrap_or_default();

    // Get the default terminal using the $TERM enviromental variable
    let term = std::env::var("TERM").unwrap_or_default();

    // Get the CPU name and manufacturer from /proc/cpuinfo using the command:
    // grep -m 1 'model name' /proc/cpuinfo
    // only take everything after the 13 character, since these are the information we want
    let cpu = String::from_utf8(
        std::process::Command::new("grep")
            .arg("-m")
            .arg("1")
            .arg("model name")
            .arg("/proc/cpuinfo")
            .output()
            .expect("Failed to execute grep")
            .stdout,
    )
    .unwrap_or_default()
    .trim()[13..]
        .to_string();

    // Get the total memory from /proc/meminfo using the command:
    // grep MemTotal /proc/meminfo
    // only take everything after the 17 character, since these are the information we want
    let memory = String::from_utf8(
        std::process::Command::new("grep")
            .arg("MemTotal")
            .arg("/proc/meminfo")
            .output()
            .expect("Failed to execute grep")
            .stdout,
    )
    .unwrap_or_default()
    .trim()[17..]
        .to_string();

    // Get the system language using the $LANG enviromental variable
    let lang = std::env::var("LANG").unwrap_or_default();

    crate::print_frog(
        user,
        hostname,
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
