// For the rust analyzer to help me

#[cfg(not(test))]
#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(not(test))]
#[cfg(target_os = "android")]
pub mod android;

#[cfg(not(test))]
#[cfg(target_os = "freebsd")]
pub mod freebsd;

#[cfg(not(test))]
#[cfg(target_os = "macos")]
pub mod macos;

#[cfg(test)]
pub mod freebsd;

#[cfg(test)]
pub mod android;

#[cfg(test)]
pub mod linux;

#[cfg(test)]
pub mod macos;

// So we don't have to spawn another external process for awk
// Allow dead code for this function since in specific targets we don't need awk
#[allow(dead_code)]
pub fn awk(str: String, start: usize, split: char, output: usize) -> String {
    if output == 0 {
        String::from(str[start..].split_once(split).unwrap_or(("", "")).0)
    } else {
        String::from(str[start..].split_once(split).unwrap_or(("", "")).1)
    }
}
