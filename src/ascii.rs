use colored::{ColoredString, Colorize};
use os_info::Type;

pub fn get_logo(os_type: Type) -> ColoredString {
    match os_type {
        Type::AIX => { return "TODO".to_string().red(); }
        Type::AlmaLinux => { return "TODO".to_string().red(); }
        Type::Alpaquita => { return "TODO".to_string().red(); }
        Type::Alpine => { return "TODO".to_string().red(); }
        Type::Amazon => { return "TODO".to_string().red(); }
        Type::Android => { return "TODO".to_string().red(); }
        Type::Arch => { return "TODO".to_string().red(); }
        Type::Artix => { return "TODO".to_string().red(); }
        Type::CentOS => { return "TODO".to_string().red(); }
        Type::Debian => {
            let debian_logo = include_str!("logos/debian.ascii");
            return debian_logo.to_string().red();
        }
        Type::DragonFly => { return "TODO".to_string().red(); }
        Type::Emscripten => { return "TODO".to_string().red(); }
        Type::EndeavourOS => { return "TODO".to_string().red(); }
        Type::Fedora => {
            let fedora_logo = include_str!("logos/fedora.ascii");
            return fedora_logo.to_string().blue();
        }
        Type::FreeBSD => { return "TODO".to_string().red(); }
        Type::Garuda => { return "TODO".to_string().red(); }
        Type::Gentoo => { return "TODO".to_string().red(); }
        Type::HardenedBSD => { return "TODO".to_string().red(); }
        Type::Illumos => { return "TODO".to_string().red(); }
        Type::Kali => { return "TODO".to_string().red(); }
        Type::Linux => { return "TODO".to_string().red(); }
        Type::Mabox => { return "TODO".to_string().red(); }
        Type::Macos => { return "TODO".to_string().red(); }
        Type::Manjaro => { return "TODO".to_string().red(); }
        Type::Mariner => { return "TODO".to_string().red(); }
        Type::MidnightBSD => { return "TODO".to_string().red(); }
        Type::Mint => { return "TODO".to_string().red(); }
        Type::NetBSD => { return "TODO".to_string().red(); }
        Type::NixOS => { return "TODO".to_string().red(); }
        Type::OpenBSD => { return "TODO".to_string().red(); }
        Type::OpenCloudOS => { return "TODO".to_string().red(); }
        Type::openEuler => { return "TODO".to_string().red(); }
        Type::openSUSE => { return "TODO".to_string().red(); }
        Type::OracleLinux => { return "TODO".to_string().red(); }
        Type::Pop => { return "TODO".to_string().red(); }
        Type::Raspbian => { return "TODO".to_string().red(); }
        Type::Redhat => { return "TODO".to_string().red(); }
        Type::RedHatEnterprise => { return "TODO".to_string().red(); }
        Type::Redox => { return "TODO".to_string().red(); }
        Type::RockyLinux => { return "TODO".to_string().red(); }
        Type::Solus => { return "TODO".to_string().red(); }
        Type::SUSE => { return "TODO".to_string().red(); }
        Type::Ubuntu => { return "TODO".to_string().red(); }
        Type::Ultramarine => { return "TODO".to_string().red(); }
        Type::Void => { return "TODO".to_string().red(); }
        Type::Unknown => { return "TODO".to_string().red(); }
        Type::Windows => {
            let windows_logo = include_str!("logos/windows.ascii");
            return windows_logo.to_string().blue();
        }
        _ => { return "TODO: error logo".to_string().red(); }
    }
}
