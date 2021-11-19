use sysinfo::{ProcessorExt, System, SystemExt};

fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();

    let mut rustfetch: Vec<String> = Vec::new();

    rustfetch.push(format!(
        "\x1B[1;92;127m       .---.`               `.---.         {}\x1B[0m@\x1B[1;92;127m{}\x1B[0m",
        std::env::var("USER").unwrap_or(String::new()),
        sys.host_name().unwrap_or(String::new())
    ));

    rustfetch.push(format!(
        "\x1B[1;92;127m    `/syhhhyso-           -osyhhhys/`      \x1B[0m-----------------------------------"
    ));
    rustfetch.push(format!(
        "\x1B[1;92;127m   .syNMdhNNhss/``.---.``/sshNNhdMNys.     OS:\x1B[0m {} {}",
        sys.name().unwrap_or(String::new()),
        std::env::var("CPUTYPE").unwrap_or(String::new())
    ));
    rustfetch.push(format!(
        "\x1B[1;92;127m   +sdMh.`+MNsssssssssssssssNM+`.hMds+     Kernel:\x1B[0m {}",
        sys.kernel_version().unwrap_or(String::new())
    ));

    let mut uptime = sys.uptime();
    let mut duration = "secs";

    if uptime > 60 {
        uptime = uptime / 60;
        duration = "mins";
    }
    if uptime > 60 {
        uptime = uptime / 60;
        duration = "hours";
    }

    rustfetch.push(format!(
        "\x1B[1;92;127m   :syNNdhNNhssssssssssssssshNNhdNNys:     Uptime:\x1B[0m {} {}",
        uptime, duration
    ));
    rustfetch.push(format!(
        "\x1B[1;92;127m    /ssyhhhysssssssssssssssssyhhhyss/      Shell:\x1B[0m {}",
        std::env::var("SHELL").unwrap_or(String::new())
    ));
    rustfetch.push(format!(
        "\x1B[1;92;127m    .ossssssssssssssssssssssssssssso.      DE:\x1B[0m {}",
        std::env::var("XDG_CURRENT_DESKTOP").unwrap_or(String::new())
    ));
    rustfetch.push(format!(
        "\x1B[1;92;127m   :sssssssssssssssssssssssssssssssss:     CPU:\x1B[0m {} {}",
        sys.global_processor_info().brand(),
        sys.global_processor_info().name()
    ));

    let mut memory = sys.total_memory();
    let mut size = "KB";

    if memory > 1024 {
        memory = memory / 1024;
        size = "MB"
    }
    if memory > 1024 {
        memory = memory / 1024;
        size = "GB"
    }

    rustfetch.push(format!(
        "\x1B[1;92;127m  /sssssssssssssssssssssssssssssssssss/    Memory:\x1B[0m {} {}",
        memory, size
    ));
    rustfetch.push(format!(
        "\x1B[1;92;127m :sssssssssssssoosssssssoosssssssssssss:   Locale:\x1B[0m {}",
        std::env::var("LANG").unwrap_or(String::new())
    ));
    rustfetch.push("\x1B[1;92;127m osssssssssssssoosssssssoossssssssssssso ".to_string());
    rustfetch.push(" osssssssssssyyyyhhhhhhhyyyyssssssssssso ".to_string());
    rustfetch.push(" /yyyyyyhhdmmmmNNNNNNNNNNNmmmmdhhyyyyyy/ ".to_string());
    rustfetch.push("  smmmNNNNNNNNNNNNNNNNNNNNNNNNNNNNNmmms  ".to_string());
    rustfetch.push("   /dNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNd/   ".to_string());
    rustfetch.push("    `:sdNNNNNNNNNNNNNNNNNNNNNNNNNds:`    ".to_string());
    rustfetch.push("       `-+shdNNNNNNNNNNNNNNNdhs+-`       ".to_string());
    rustfetch.push("             `.-:///////:-.`             \x1B[0m".to_string());

    rustfetch.iter().for_each(|line| println!("{}", line));
}
