#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]

#[cfg(feature = "axstd")]
use axstd::println;
use axstd::thread::sleep;
use axstd::time::Duration;

#[cfg_attr(feature = "axstd", unsafe(no_mangle))]
fn main() {
    loop{
        println!("Hello, world!");
        sleep(Duration::from_secs(3));
    }
}
