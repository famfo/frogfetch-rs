#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(target_os = "android")]
pub mod android;

#[cfg(target_os = "bsd")]
pub mod bsd;
