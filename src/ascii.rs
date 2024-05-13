use colored::{ColoredString, Colorize};
use os_info::Type;

pub fn get_logo(os_type: Type) -> ColoredString {
    match os_type {
        Type::AIX => { "TODO".to_string().red() }
        Type::AlmaLinux => { "TODO".to_string().red() }
        Type::Alpaquita => { "TODO".to_string().red() }
        Type::Alpine => { "TODO".to_string().red() }
        Type::Amazon => { "TODO".to_string().red() }
        Type::Android => { "TODO".to_string().red() }
        Type::Arch => { "TODO".to_string().red() }
        Type::Artix => { "TODO".to_string().red() }
        Type::CentOS => { "TODO".to_string().red() }
        Type::Debian => {
            let debian_logo = include_str!("logos/debian.ascii");
            debian_logo.to_string().red()
        }
        Type::DragonFly => { "TODO".to_string().red() }
        Type::Emscripten => { "TODO".to_string().red() }
        Type::EndeavourOS => { "TODO".to_string().red() }
        Type::Fedora => {
            let fedora_logo = include_str!("logos/fedora.ascii");
            fedora_logo.to_string().blue()
        }
        Type::FreeBSD => { "TODO".to_string().red() }
        Type::Garuda => { "TODO".to_string().red() }
        Type::Gentoo => { "TODO".to_string().red() }
        Type::HardenedBSD => { "TODO".to_string().red() }
        Type::Illumos => { "TODO".to_string().red() }
        Type::Kali => { "TODO".to_string().red() }
        Type::Linux => { "TODO".to_string().red() }
        Type::Mabox => { "TODO".to_string().red() }
        Type::Macos => { "TODO".to_string().red() }
        Type::Manjaro => { "TODO".to_string().red() }
        Type::Mariner => { "TODO".to_string().red() }
        Type::MidnightBSD => { "TODO".to_string().red() }
        Type::Mint => { "TODO".to_string().red() }
        Type::NetBSD => { "TODO".to_string().red() }
        Type::NixOS => { "TODO".to_string().red() }
        Type::OpenBSD => { "TODO".to_string().red() }
        Type::OpenCloudOS => { "TODO".to_string().red() }
        Type::openEuler => { "TODO".to_string().red() }
        Type::openSUSE => { "TODO".to_string().red() }
        Type::OracleLinux => { "TODO".to_string().red() }
        Type::Pop => { "TODO".to_string().red() }
        Type::Raspbian => { "TODO".to_string().red() }
        Type::Redhat => { "TODO".to_string().red() }
        Type::RedHatEnterprise => { "TODO".to_string().red() }
        Type::Redox => { "TODO".to_string().red() }
        Type::RockyLinux => { "TODO".to_string().red() }
        Type::Solus => { "TODO".to_string().red() }
        Type::SUSE => { "TODO".to_string().red() }
        Type::Ubuntu => {
            let ubuntu_logo = include_str!("logos/ubuntu.ascii");
            ubuntu_logo.to_string().truecolor(255, 150, 0)
        }
        Type::Ultramarine => { "TODO".to_string().red() }
        Type::Void => { "TODO".to_string().red() }
        Type::Unknown => { "TODO".to_string().red() }
        Type::Windows => {
            let windows_logo = include_str!("logos/windows.ascii");
            windows_logo.to_string().blue()
        }
        _ => { "TODO: error logo".to_string().red() }
    }
}
