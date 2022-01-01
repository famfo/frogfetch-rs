// Allow some clippy warnings since I don't know how to write the stuff differently
// Allow too_many_arguments because I want to pass all the values to print_frog in one
// Allow needless_pass_by_value because I get my values with String::from_utf8()
// Allow too many lines because it's a stupid lint
#![allow(
    clippy::too_many_arguments,
    clippy::needless_pass_by_value,
    clippy::too_many_lines
)]

mod os_specific;

fn main() {
    #[cfg(target_os = "linux")]
    os_specific::linux::get_info();

    #[cfg(target_os = "android")]
    os_specific::android::get_info();

    #[cfg(target_os = "freebsd")]
    os_specific::freebsd::get_info();

    #[cfg(target_os = "windows")]
    os_specific::windows::get_info();

    // TODO: Get it to work on Windows, MacOS, and other BSDs
    #[cfg(not(any(
        target_os = "linux",
        target_os = "android",
        target_os = "freebsd",
        target_os = "windows"
    )))]
    todo!();
}

fn print_frog(
    user: String,
    hostname: String,
    os: String,
    architecture: String,
    kernel: String,
    uptime: String,
    shell: String,
    term: String,
    cpu: String,
    memory: String,
    lang: String,
) {
    let mut frogfetch: Vec<String> = Vec::new();

    frogfetch.push(format!(
        "\x1B[1;92;127m       .---.`               `.---.         {}\x1B[0m@\x1B[1;92;127m{}\x1B[0m",
        user,
        hostname,
    ));
    frogfetch.push(
        "\x1B[1;92;127m    `/syhhhyso-           -osyhhhys/`      \x1B[0m-----------------------------------".to_string());
    frogfetch.push(format!(
        "\x1B[1;92;127m   .syNMdhNNhss/``.---.``/sshNNhdMNys.     OS:\x1B[0m {} {}",
        os, architecture,
    ));
    frogfetch.push(format!(
        "\x1B[1;92;127m   +sdMh.`+MNsssssssssssssssNM+`.hMds+     Kernel:\x1B[0m {}",
        kernel,
    ));
    frogfetch.push(format!(
        "\x1B[1;92;127m   :syNNdhNNhssssssssssssssshNNhdNNys:     Uptime:\x1B[0m {}",
        uptime,
    ));
    frogfetch.push(format!(
        "\x1B[1;92;127m    /ssyhhhysssssssssssssssssyhhhyss/      Shell:\x1B[0m {}",
        shell,
    ));
    frogfetch.push(format!(
        "\x1B[1;92;127m    .ossssssssssssssssssssssssssssso.      Terminal:\x1B[0m {}",
        term,
    ));
    frogfetch.push(format!(
        "\x1B[1;92;127m   :sssssssssssssssssssssssssssssssss:     CPU:\x1B[0m {}",
        cpu,
    ));

    frogfetch.push(format!(
        "\x1B[1;92;127m  /sssssssssssssssssssssssssssssssssss/    System memory:\x1B[0m {}",
        memory,
    ));
    frogfetch.push(format!(
        "\x1B[1;92;127m :sssssssssssssoosssssssoosssssssssssss:   Locale:\x1B[0m {}",
        lang,
    ));
    frogfetch.push("\x1B[1;92;127m osssssssssssssoosssssssoossssssssssssso ".to_string());
    frogfetch.push(" osssssssssssyyyyhhhhhhhyyyyssssssssssso ".to_string());
    frogfetch.push(" /yyyyyyhhdmmmmNNNNNNNNNNNmmmmdhhyyyyyy/ ".to_string());
    frogfetch.push("  smmmNNNNNNNNNNNNNNNNNNNNNNNNNNNNNmmms  ".to_string());
    frogfetch.push("   /dNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNd/   ".to_string());
    frogfetch.push("    `:sdNNNNNNNNNNNNNNNNNNNNNNNNNds:`    ".to_string());
    frogfetch.push("       `-+shdNNNNNNNNNNNNNNNdhs+-`       ".to_string());
    frogfetch.push("             `.-:///////:-.`             \x1B[0m".to_string());

    for line in frogfetch {
        println!("{}", line);
    }
}
