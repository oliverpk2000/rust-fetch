use os_info::Type;

pub fn fmt_info() -> String {
    let info = os_info::get();

    let os_type = info.os_type();
    let version = info.version();
    let edition = info.edition().unwrap_or("unknown/error");
    let codename = info.codename().unwrap_or("unknown/error");
    let bitness = info.bitness();
    let architecture = info.architecture().unwrap_or("unknown/error");

    return format!("| {os_type} | {version} | {edition} | {codename} | {bitness} | {architecture} |");
}
