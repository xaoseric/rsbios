#![feature(lang_items)]
#![no_main]

extern crate cpuid;

mod metadata;

use metadata::bios_metadata::BIOSMetadata;

pub extern fn rust_start(_bist: u32) {
    let bios_meta: BIOSMetadata;

    // detect what cpu we are running on and initialize bios metadata
    match cpuid::identify() {
        Ok(info) => {
            bios_meta = BIOSMetadata {
                version_major: 0,
                version_minor: 0,
                version_patch: 1,

                pre_release: false,
                manufacturer: "Test".to_string(),
                cpu: info
            }
        },
        Err(_error) => {
            
        }
    }

    loop {

    }
}