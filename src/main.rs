mod os_specific;

fn main() {
    #![cfg(any(target_os = "linux", target_os = "android"))]
    os_specific::linux_android::get_info();

    // TODO: Get it to work on Windows, MacOS, Android and BSD
    #[cfg(not(any(target_os = "linux", target_os = "android")))]
    unimplemented!();
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
    let mut rustfetch: Vec<String> = Vec::new();

    rustfetch.push(format!(
        "\x1B[1;92;127m       .---.`               `.---.         {}\x1B[0m@\x1B[1;92;127m{}\x1B[0m",
        user,
        hostname,
    ));
    rustfetch.push(format!(
        "\x1B[1;92;127m    `/syhhhyso-           -osyhhhys/`      \x1B[0m-----------------------------------"
    ));
    rustfetch.push(format!(
        "\x1B[1;92;127m   .syNMdhNNhss/``.---.``/sshNNhdMNys.     OS:\x1B[0m {} {}",
        os, architecture,
    ));
    rustfetch.push(format!(
        "\x1B[1;92;127m   +sdMh.`+MNsssssssssssssssNM+`.hMds+     Kernel:\x1B[0m {}",
        kernel,
    ));
    rustfetch.push(format!(
        "\x1B[1;92;127m   :syNNdhNNhssssssssssssssshNNhdNNys:     Uptime:\x1B[0m {}",
        uptime,
    ));
    rustfetch.push(format!(
        "\x1B[1;92;127m    /ssyhhhysssssssssssssssssyhhhyss/      Shell:\x1B[0m {}",
        shell,
    ));
    rustfetch.push(format!(
        "\x1B[1;92;127m    .ossssssssssssssssssssssssssssso.      Terminal:\x1B[0m {}",
        term,
    ));
    rustfetch.push(format!(
        "\x1B[1;92;127m   :sssssssssssssssssssssssssssssssss:     CPU:\x1B[0m {}",
        cpu,
    ));

    rustfetch.push(format!(
        "\x1B[1;92;127m  /sssssssssssssssssssssssssssssssssss/    System memory:\x1B[0m {}",
        memory,
    ));
    rustfetch.push(format!(
        "\x1B[1;92;127m :sssssssssssssoosssssssoosssssssssssss:   Locale:\x1B[0m {}",
        lang,
    ));
    rustfetch.push("\x1B[1;92;127m osssssssssssssoosssssssoossssssssssssso".to_string());
    rustfetch.push(" osssssssssssyyyyhhhhhhhyyyyssssssssssso ".to_string());
    rustfetch.push(" /yyyyyyhhdmmmmNNNNNNNNNNNmmmmdhhyyyyyy/ ".to_string());
    rustfetch.push("  smmmNNNNNNNNNNNNNNNNNNNNNNNNNNNNNmmms  ".to_string());
    rustfetch.push("   /dNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNd/   ".to_string());
    rustfetch.push("    `:sdNNNNNNNNNNNNNNNNNNNNNNNNNds:`    ".to_string());
    rustfetch.push("       `-+shdNNNNNNNNNNNNNNNdhs+-`       ".to_string());
    rustfetch.push("             `.-:///////:-.`             \x1B[0m".to_string());

    rustfetch.iter().for_each(|line| println!("{}", line));
}
