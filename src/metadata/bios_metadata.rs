extern crate cpuid;

/// Contains information about the base system and devices
pub struct BIOSMetadata {
    // BIOS version (major).
    pub version_major: u16,

    // BIOS version (minor).
    pub version_minor: u16,

    // BIOS version (patch)
    pub version_patch: u16,

    // Whether the BIOS version is a pre-release
    pub pre_release: bool,

    // manufacturer of the computer
    pub manufacturer: String,

    pub cpu: cpuid::CpuInfo
}