use sysinfo::{ProcessorExt, System, SystemExt};

fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();

    let mut rustfetch: Vec<String> = Vec::new();

    rustfetch.push(format!(
        "       .---.`               `.---.         {}@{}",
        std::env::var("USER").unwrap_or(String::new()),
        sys.host_name().unwrap_or(String::new())
    ));

    rustfetch.push(format!(
        "    `/syhhhyso-           -osyhhhys/`      -----------------------------------"
    ));
    rustfetch.push(format!(
        "   .syNMdhNNhss/``.---.``/sshNNhdMNys.     OS: {} {}",
        sys.name().unwrap_or(String::new()),
        std::env::var("CPUTYPE").unwrap_or(String::new())
    ));
    rustfetch.push(format!(
        "   +sdMh.`+MNsssssssssssssssNM+`.hMds+     Kernel: {}",
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
        "   :syNNdhNNhssssssssssssssshNNhdNNys:     Uptime: {} {}",
        uptime, duration
    ));
    rustfetch.push(format!(
        "    /ssyhhhysssssssssssssssssyhhhyss/      Shell: {}",
        std::env::var("SHELL").unwrap_or(String::new())
    ));
    rustfetch.push(format!(
        "    .ossssssssssssssssssssssssssssso.      DE: {}",
        std::env::var("XDG_CURRENT_DESKTOP").unwrap_or(String::new())
    ));
    rustfetch.push(format!(
        "   :sssssssssssssssssssssssssssssssss:     CPU: {} {}",
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
        "  /sssssssssssssssssssssssssssssssssss/    Memory: {} {}",
        memory, size
    ));
    rustfetch.push(format!(
        " :sssssssssssssoosssssssoosssssssssssss:   Locale: {}",
        std::env::var("LANG").unwrap_or(String::new())
    ));
    rustfetch.push(" osssssssssssssoosssssssoossssssssssssso ".to_string());
    rustfetch.push(" osssssssssssyyyyhhhhhhhyyyyssssssssssso ".to_string());
    rustfetch.push(" /yyyyyyhhdmmmmNNNNNNNNNNNmmmmdhhyyyyyy/ ".to_string());
    rustfetch.push("  smmmNNNNNNNNNNNNNNNNNNNNNNNNNNNNNmmms  ".to_string());
    rustfetch.push("   /dNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNd/   ".to_string());
    rustfetch.push("    `:sdNNNNNNNNNNNNNNNNNNNNNNNNNds:`    ".to_string());
    rustfetch.push("       `-+shdNNNNNNNNNNNNNNNdhs+-`       ".to_string());
    rustfetch.push("             `.-:///////:-.`             ".to_string());

    rustfetch.iter().for_each(|line| println!("{}", line));
}
