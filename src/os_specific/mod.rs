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

#[cfg(test)]
pub mod freebsd;

#[cfg(test)]
pub mod android;

#[cfg(test)]
pub mod linux;
