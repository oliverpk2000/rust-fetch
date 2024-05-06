use os_info::Type;

pub fn get_logo(os_type: Type) -> String {
    match os_type {
        Type::AIX => { return "TODO".to_string(); }
        Type::AlmaLinux => { return "TODO".to_string(); }
        Type::Alpaquita => { return "TODO".to_string(); }
        Type::Alpine => { return "TODO".to_string(); }
        Type::Amazon => { return "TODO".to_string(); }
        Type::Android => { return "TODO".to_string(); }
        Type::Arch => { return "TODO".to_string(); }
        Type::Artix => { return "TODO".to_string(); }
        Type::CentOS => { return "TODO".to_string(); }
        Type::Debian => { return "TODO".to_string(); }
        Type::DragonFly => { return "TODO".to_string(); }
        Type::Emscripten => { return "TODO".to_string(); }
        Type::EndeavourOS => { return "TODO".to_string(); }
        Type::Fedora => {
            let fedora_logo = include_str!("logos/fedora.ascii");
            return fedora_logo.to_string();
        }
        Type::FreeBSD => { return "TODO".to_string(); }
        Type::Garuda => { return "TODO".to_string(); }
        Type::Gentoo => { return "TODO".to_string(); }
        Type::HardenedBSD => { return "TODO".to_string(); }
        Type::Illumos => { return "TODO".to_string(); }
        Type::Kali => { return "TODO".to_string(); }
        Type::Linux => { return "TODO".to_string(); }
        Type::Mabox => { return "TODO".to_string(); }
        Type::Macos => { return "TODO".to_string(); }
        Type::Manjaro => { return "TODO".to_string(); }
        Type::Mariner => { return "TODO".to_string(); }
        Type::MidnightBSD => { return "TODO".to_string(); }
        Type::Mint => { return "TODO".to_string(); }
        Type::NetBSD => { return "TODO".to_string(); }
        Type::NixOS => { return "TODO".to_string(); }
        Type::OpenBSD => { return "TODO".to_string(); }
        Type::OpenCloudOS => { return "TODO".to_string(); }
        Type::openEuler => { return "TODO".to_string(); }
        Type::openSUSE => { return "TODO".to_string(); }
        Type::OracleLinux => { return "TODO".to_string(); }
        Type::Pop => { return "TODO".to_string(); }
        Type::Raspbian => { return "TODO".to_string(); }
        Type::Redhat => { return "TODO".to_string(); }
        Type::RedHatEnterprise => { return "TODO".to_string(); }
        Type::Redox => { return "TODO".to_string(); }
        Type::RockyLinux => { return "TODO".to_string(); }
        Type::Solus => { return "TODO".to_string(); }
        Type::SUSE => { return "TODO".to_string(); }
        Type::Ubuntu => { return "TODO".to_string(); }
        Type::Ultramarine => { return "TODO".to_string(); }
        Type::Void => { return "TODO".to_string(); }
        Type::Unknown => { return "TODO".to_string(); }
        Type::Windows => {
            let windows_logo = include_str!("logos/windows.ascii");
            return windows_logo.to_string();
        }
        _ => { return "TODO: error logo".to_string(); }
    }
}
