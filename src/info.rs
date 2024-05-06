use os_info::Type;

pub fn fmt_info_header() -> String {
    return format!("| {0: <15} | {1: <15} | {2: <20} | {3: <15} | {4: <10} | {5: <15} |",
                   "os_type",
                   "version",
                   "edition",
                   "codename",
                   "bitness",
                   "architecture");
}

pub fn fmt_info() -> String {
    let info = os_info::get();

    let os_type = info.os_type();
    let version = info.version();
    let edition = info.edition().unwrap_or("unknown/error");
    let codename = info.codename().unwrap_or("unknown/error");
    let bitness = info.bitness();
    let architecture = info.architecture().unwrap_or("unknown/error");

    return format!("| {0: <15} | {1: <15} | {2: <20} | {3: <15} | {4: <10} | {5: <15} |",
                   os_type.to_string(),
                   version.to_string(),
                   edition,
                   codename,
                   bitness.to_string(),
                   architecture);
}

pub fn get_os_type() -> Type {
    let info = os_info::get();
    let os_type = info.os_type();
    return os_type;
}