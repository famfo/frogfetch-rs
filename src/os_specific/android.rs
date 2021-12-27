// We can probably make this more efficient not using trim().to_string() all the time

use std::process::Command;

pub fn get_info() {
    // Get the current user using the $USER enviromental variable
    let user = String::from_utf8(
        Command::new("whoami")
            .output()
            .expect("Failed to execute whoami")
            .stdout,
    )
    .unwrap_or_default()
    .trim()
    .to_string();

    // Get the hostname using hostname
    let hostname = String::from_utf8(
        Command::new("hostname")
            .output()
            .expect("Failed to execute hostname")
            .stdout,
    )
    .unwrap_or_default()
    .trim()
    .to_string();

    // Get the os using uname -s
    let os = format!(
        "{} {}",
        String::from_utf8(
            Command::new("uname")
                .arg("-o")
                .output()
                .expect("Failed to execute uname -o")
                .stdout
        )
        .unwrap_or_default()
        .trim(),
        String::from_utf8(
            Command::new("getprop")
                .arg("ro.build.version.release")
                .output()
                .expect("Failed to execute getprop")
                .stdout
        )
        .unwrap_or_default()
        .trim(),
    );

    // Get the architecture using uname -m
    let architecture = String::from_utf8(
        Command::new("uname")
            .arg("-m")
            .output()
            .expect("Failed to execute uname -m")
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
            .expect("Failed to execute uname -r")
            .stdout,
    )
    .unwrap_or_default()
    .trim()
    .to_string();

    // Get the uptime using the uptime command
    let up_uptime = String::from_utf8(
        Command::new("uptime")
            .arg("-p")
            .output()
            .expect("Failed to execute uptime -p")
            .stdout,
    )
    .unwrap_or_default()
    .trim()
    .to_string();

    // Trim out the first 3 chars of the uptime output (up )
    let uptime = &up_uptime.as_str()[3..];

    // Get the default shell using the $SHELL enviromental variable
    let shell = std::env::var("SHELL").unwrap_or_default();

    // Get the default terminal using the $TERM enviromental variable
    let term = std::env::var("TERM").unwrap_or_default();

    // Get the CPU name and manufacturer from /proc/cpuinfo using the command:
    // grep -m 1 'model name' /proc/cpuinfo | awk -F: '{ print $2 }'
    let lscpu0 = std::process::Command::new("lscpu")
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to execute lscpu")
        .stdout
        .expect("Failed to open grep stdout");

    let grep0 = std::process::Command::new("grep")
        .arg("Vendor ID")
        .stdin(std::process::Stdio::from(lscpu0))
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to execute grep")
        .stdout
        .expect("Failed to open grep stdout");

    let awk0 = std::process::Command::new("awk")
        .arg("-F:")
        .arg("{ print $2 }")
        .stdin(std::process::Stdio::from(grep0))
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to execute awk");

    let lscpu1 = std::process::Command::new("lscpu")
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to execute lscpu")
        .stdout
        .expect("Failed to open grep stdout");

    let grep1 = std::process::Command::new("grep")
        .arg("Model name")
        .stdin(std::process::Stdio::from(lscpu1))
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to execute grep")
        .stdout
        .expect("Failed to open grep stdout");

    let awk1 = std::process::Command::new("awk")
        .arg("-F:")
        .arg("{ print $2 }")
        .stdin(std::process::Stdio::from(grep1))
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to execute awk");

    let cpu = format!(
        "{} {}",
        String::from_utf8(
            awk0.wait_with_output()
                .expect("Failed to wait on awk")
                .stdout
                .as_slice()
                .to_vec(),
        )
        .unwrap_or_default()
        .trim(),
        String::from_utf8(
            awk1.wait_with_output()
                .expect("Failed to wait on awk")
                .stdout
                .as_slice()
                .to_vec(),
        )
        .unwrap_or_default()
        .trim()
    );

    // Get the total memory from /proc/meminfo using the command:
    // grep MemTotal /proc/meminfo | awk -F: '{ print $2 }'
    let grep = std::process::Command::new("grep")
        .arg("MemTotal")
        .arg("/proc/meminfo")
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to execute grep")
        .stdout
        .expect("Failed to open grep stdout");

    let awk = std::process::Command::new("awk")
        .arg("-F:")
        .arg("{ print $2 }")
        .stdin(std::process::Stdio::from(grep))
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to execute awk");

    let memory = String::from_utf8(
        awk.wait_with_output()
            .expect("Failed to wait on awk")
            .stdout
            .as_slice()
            .to_vec(),
    )
    .unwrap_or_default()
    .trim()
    .to_string();

    // Get the system language using the $LANG enviromental variable
    let lang = std::env::var("LANG").unwrap_or_default();

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
