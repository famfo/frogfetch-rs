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
            .expect("Failed to execute uname -m")
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
    let uptime = "";

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
            .expect("Failed to execute uptime -p")
            .stdout
            .to_vec(),
    )
    .unwrap_or(String::new())
    .trim()
    .to_string();

    // Get the total memory in byte using sysctl -n hw.physmem
    let memory = (std::str::FromStr::from_str(
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
