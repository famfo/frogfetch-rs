// We can probably make this more efficient not using trim().to_string() all the time

use std::process::Command;

pub fn get_info() {
    // Get the current user using the $USER enviromental variable
    let user = std::env::var("USER").unwrap_or(String::new());

    // Get the hostname using hostname
    let hostname = String::from_utf8(
        Command::new("hostname")
            .output()
            .expect("Failed to execute hostname")
            .stdout
            .to_vec(),
    )
    .unwrap_or(String::new())
    .trim()
    .to_string();

    // Get the os using sysctl -n kern.ostype
    let os = String::from_utf8(
        Command::new("sysctl")
            .arg("-n")
            .arg("kern.ostype")
            .output()
            .expect("Failed to execute sysctl")
            .stdout
            .to_vec(),
    )
    .unwrap_or(String::new())
    .trim()
    .to_string();

    // Get the architecture using uname -m
    let architecture = String::from_utf8(
        Command::new("uname")
            .arg("-m")
            .output()
            .expect("Failed to execute uname")
            .stdout
            .to_vec(),
    )
    .unwrap_or(String::new())
    .trim()
    .to_string();

    // Get the kernel name using sysctl -n kern.osrelease
    let kernel = String::from_utf8(
        Command::new("sysctl")
            .arg("-n")
            .arg("kern.osrelease")
            .output()
            .expect("Failed to execute sysctl")
            .stdout
            .to_vec(),
    )
    .unwrap_or(String::new())
    .trim()
    .to_string();

    // UPTIME: TODO
    let uptime = String::from_utf8(
        Command::new("sysctl")
            .arg("-n")
            .arg("kern.boottime")
            .output()
            .expect("Failed to execute sysctl")
            .stdout
            .to_vec(),
    )
    .unwrap_or(String::new())
    .trim()
    .to_string();

    let current_time = std::str::FromStr::from_str(
        String::from_utf8(
            Command::new("date")
                .arg("+%s")
                .output()
                .expect("Failed to execute date")
                .stdout
                .to_vec(),
        )
        .unwrap_or(String::new())
        .trim(),
    )
    .unwrap_or(0);

    let uptime = current_time
        - std::str::FromStr::from_str(uptime[8..].split_once(',').unwrap_or(("", "")).0)
            .unwrap_or(current_time);

    let days = (uptime / 60 / 60 / 24) as u32;
    let hours = (uptime / 60 / 60) as u32;
    let minutes = (uptime / 60 % 60) as u32;

    let uptime = {
        if days != 0 {
            format!("{} days, {} hours, {} minutes", days, hours, minutes)
        } else if hours != 0 {
            format!("{} hours, {} minutes", hours, minutes)
        } else {
            format!("{} minutes", minutes)
        }
    };

    // Get the default shell using the $SHELL enviromental variable
    let shell = std::env::var("SHELL").unwrap_or(String::new());

    // Get the default terminal using the $TERM enviromental variable
    let term = std::env::var("TERM").unwrap_or(String::new());

    // Get the CPU manufacturer and model using the sysctl -n hw.model command
    let cpu = String::from_utf8(
        Command::new("sysctl")
            .arg("-n")
            .arg("hw.model")
            .output()
            .expect("Failed to execute sysctl")
            .stdout
            .to_vec(),
    )
    .unwrap_or(String::new())
    .trim()
    .to_string();

    // Get the total memory in byte using sysctl -n hw.physmem
    let mut memory = (std::str::FromStr::from_str(
        String::from_utf8(
            Command::new("sysctl")
                .arg("-n")
                .arg("hw.physmem")
                .output()
                .expect("Failed to execute sysctl")
                .stdout
                .to_vec(),
        )
        .unwrap_or(String::new())
        .trim(),
    )
    .unwrap_or(0)
        / 1024)
        .to_string();

    memory.push_str(" kB");

    // Get the system language using the $LANG enviromental variable
    let lang = std::env::var("LANG").unwrap_or(String::new());

    crate::print_frog(
        user,
        hostname,
        os,
        architecture,
        kernel,
        uptime.to_string(),
        shell,
        term,
        cpu,
        memory,
        lang,
    );
}
